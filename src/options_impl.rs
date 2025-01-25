#[cfg(doc)]
use serde::de::{Deserializer, EnumAccess, MapAccess};

use crate::fallback::DefaultFallbacks;
pub use crate::random_trailer::RandomTrailer;
use crate::random_trailer::{NoopRandomTrailer, StringLike};
use crate::unstable::{DefaultReporter, ExtraOptionsIsUnstable};
use crate::Options;

impl<Extra: ExtraOptions> Options<Extra> {
    /// Do our best to take off any potential junk that was only added by us,
    /// such as the JSON-specific trick stuff.
    ///
    /// Return true if the input was modified and this value seems to be incomplete.
    #[must_use]
    pub(crate) fn remove_tag_from_stringlike(&self, stringy: &mut impl StringLike) -> bool {
        #![cfg_attr(not(feature = "rand"), allow(unused_variables))]

        #[cfg(feature = "rand")]
        {
            if let Some(tag) = self.random_tag.as_ref() {
                return self.extra.get_random_trailer().remove_trailer(stringy, tag);
            }
        }

        false
    }
}

/// Monomorphized options.
///
/// This is a type parameter pack.
///
/// All of this is unstable.
///
/// ## Parameters
///
/// The `'error` lifetime parameter is the lifetime of deserializer errors.
/// This matters for the default reporter, which logs errors with [`tracing`],
/// and [`tracing`] only seems to accept `&(dyn std::error::Error + 'static)`. So
/// [`DefaultExtraOptions`] only implements [`ExtraOptions`].
///
/// An alternative reporter could instead always log e.g. the display representation
/// of the error. Then the corresponding parameter pack could implement [`ExtraOptions<'_>`].
pub trait ExtraOptions: ExtraOptionsIsUnstable {
    /// Will only be called once per invocation of a public function in this crate
    fn make_reporter(&mut self) -> Self::Reporter;
    type Reporter: crate::reporter::Reporter;

    /// Will only be called once per invocation of a public function in this crate
    fn make_fallback_provider(
        &mut self,
        behavior: &UnstableCustomBehavior,
    ) -> Self::FallbackProvider;
    type FallbackProvider: crate::fallback::Fallbacks;

    fn get_random_trailer(&self) -> &Self::RandomTrailer;
    type RandomTrailer: RandomTrailer;
}

pub type DefaultExtraOptions =
    ExtraOptionsStruct<MakeDefaultReporter, MakeDefaultFallbacks, NoopRandomTrailer>;
#[cfg(all(feature = "rand", feature = "serde_json"))]
pub type JsonExtraOptions = ExtraOptionsStruct<
    MakeDefaultReporter,
    MakeDefaultFallbacks,
    crate::random_trailer::json::JsonRandomTrailer,
>;
#[cfg(all(feature = "rand", feature = "serde_yaml"))]
pub type YamlExtraOptions = ExtraOptionsStruct<
    MakeDefaultReporter,
    MakeDefaultFallbacks,
    crate::random_trailer::yaml::YamlRandomTrailer,
>;

#[derive(Debug, Clone, Default)]
pub struct ExtraOptionsStruct<MakeReporter, MakeFallbackProvider, RandomTrailer> {
    pub(crate) make_reporter: MakeReporter,
    pub(crate) make_fallback_provider: MakeFallbackProvider,
    pub(crate) random_trailer: RandomTrailer,
}

pub trait MakeReporter {
    type Reporter: crate::reporter::Reporter;
    fn make_reporter(&mut self) -> Self::Reporter;
}
pub trait MakeFallbackProvider {
    type FallbackProvider: crate::fallback::Fallbacks;
    fn make_fallback_provider(
        &mut self,
        behavior: &UnstableCustomBehavior,
    ) -> Self::FallbackProvider;
}

#[derive(Debug, Clone, Default)]
pub struct MakeDefaultReporter;
impl MakeReporter for MakeDefaultReporter {
    type Reporter = DefaultReporter;
    fn make_reporter(&mut self) -> Self::Reporter {
        DefaultReporter::new()
    }
}
#[derive(Debug, Clone, Default)]
pub struct MakeDefaultFallbacks;
impl MakeFallbackProvider for MakeDefaultFallbacks {
    type FallbackProvider = DefaultFallbacks;
    fn make_fallback_provider(
        &mut self,
        behavior: &UnstableCustomBehavior,
    ) -> Self::FallbackProvider {
        DefaultFallbacks {
            behavior: behavior.clone(),
        }
    }
}

impl<R, F, RT> ExtraOptions for ExtraOptionsStruct<R, F, RT>
where
    R: MakeReporter,
    F: MakeFallbackProvider,
    RT: RandomTrailer,
{
    fn make_reporter(&mut self) -> Self::Reporter {
        self.make_reporter.make_reporter()
    }
    type Reporter = R::Reporter;

    fn make_fallback_provider(
        &mut self,
        behavior: &UnstableCustomBehavior,
    ) -> Self::FallbackProvider {
        self.make_fallback_provider.make_fallback_provider(behavior)
    }
    type FallbackProvider = F::FallbackProvider;

    fn get_random_trailer(&self) -> &RT {
        &self.random_trailer
    }
    type RandomTrailer = RT;
}

impl<R, F, RT> ExtraOptionsIsUnstable for ExtraOptionsStruct<R, F, RT> {}

/// Customize behavior.
///
/// This both controls
///
///   - when deserializer encounters end-of-input, but we still have
///     a chance to fill in the value and succeed deserialization, then
///     we can make an educated guess based on what the data type expected
///     (which method of [`Deserializer`] was called).
///
///     For instance, when deserializing an option, JSON `fa` will choose the
///     `Some` case, the deserializer will error, but can save deserialization
///     and fill in `none`.
///
///   - which points are eligible for backtracking, e.g. is it okay to retry
///     but omit a list item, a map item, or convert `Some` into `None`.
///
/// The default is an educated guess that should make many data types deserialize
/// successfully on the first backtrack, without trying to apply too many
/// fallbacks: incomplete list items are skipped, but no fallback numbers are
/// filled in.
///
/// ## Limitations
///
/// Consider that fallback values are not a good substitute for `#[serde(default)]`:
///
/// ```
/// struct Point {
///     // note: missing #[serde(default)]
///     x: i32,
///     // note: missing #[serde(default)]
///     y: i32,
/// }
/// ```
///
/// We could put a fallback of `0` for integers, and that will make `{"x": 1, "y":` parse successfully,
/// but `{"x": 1` will still not parse because all `Point`s require a value for `y`.
///
/// ## Stability
///
/// This interface is not subject to semver (as it is unstable), and may change
/// or be removed at any time.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UnstableCustomBehavior {
    /// If the visitor returns Ok but the deserializer returns an error, then return
    /// the visitor's value.
    pub tolerate_deserializer_fail_after_visit_success: bool,
    pub fallback_any_as_none: bool,
    pub fallback_ignored_any_as_none: bool,
    pub fallback_default_bool: Option<bool>,
    pub fallback_int_zero: bool,
    pub fallback_default_float: Option<f32>,
    pub fallback_default_char: Option<char>,
    pub fallback_default_str: Option<&'static str>,
    pub fallback_bytes_empty: bool,
    /// In case [`Deserializer::deserialize_option`] does not do anything, then
    /// just go in and visit a None.
    pub fallback_none: bool,
    pub fallback_none_at_mandatory: bool,
    /// In case [`Deserializer::deserialize_unit`] does not do anything, then
    /// just go in and visit a unit.
    ///
    /// ## Caveat
    ///
    /// In combination with the JSON string trick, this can sometimes generate spurious
    /// list elements.
    pub fallback_unit: bool,
    pub fallback_unit_at_mandatory: bool,

    /// In case [`Deserializer::deserialize_unit_struct`] does not do anything, then
    /// just go in and visit a unit struct.
    ///
    /// ## Caveat
    ///
    /// In combination with the JSON string trick, this can sometimes generate spurious
    /// list elements.
    pub fallback_unit_struct: bool,
    pub fallback_unit_struct_at_mandatory: bool,

    pub fallback_seq_empty: bool,
    pub fallback_seq_empty_at_root: bool,
    pub backtrack_seq_empty_for_value: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_seq_skip_item: bool,
    pub backtrack_seq_skip_item: bool,
    pub fallback_tuple_empty: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_tuple_skip_item: bool,
    pub backtrack_tuple_skip_item: bool,
    pub fallback_tuple_struct_empty: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_tuple_struct_skip_item: bool,
    pub backtrack_tuple_struct_skip_item: bool,
    pub fallback_map_empty: bool,
    pub fallback_map_empty_at_root: bool,
    pub backtrack_map_empty_for_value: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_map_skip_item: bool,
    pub backtrack_map_skip_item: bool,
    pub fallback_struct_empty: bool,
    pub fallback_struct_empty_at_root: bool,
    pub backtrack_struct_empty_for_value: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_struct_skip_field: bool,
    pub backtrack_struct_skip_field: bool,
    pub fallback_unit_variant: bool,

    /// Whether it's okay to fallback to skipping an element or field in circumstances
    /// other than [`Deserializer::deserialize_seq`] or [`Deserializer::deserialize_tuple`].
    ///
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_other_skip_item: bool,
    pub backtrack_other_skip_item: bool,

    /// Whether incomplete strings should be allowed or rejected
    /// in [`MapAccess::next_key`] or [`EnumAccess::variant`].
    pub allow_incomplete_string_in_key_or_variant: bool,
}

impl Default for UnstableCustomBehavior {
    fn default() -> Self {
        Self {
            tolerate_deserializer_fail_after_visit_success: true,
            fallback_any_as_none: false,
            fallback_ignored_any_as_none: false,
            fallback_default_bool: None,
            fallback_int_zero: false,
            fallback_default_float: None,
            fallback_default_char: None,
            fallback_default_str: None,
            fallback_bytes_empty: false,
            fallback_none: true,
            fallback_none_at_mandatory: true,
            fallback_unit: false,
            fallback_unit_at_mandatory: true,
            fallback_unit_struct: false,
            fallback_unit_struct_at_mandatory: true,
            fallback_seq_empty: false,
            fallback_seq_empty_at_root: true,
            backtrack_seq_empty_for_value: true,
            fallback_seq_skip_item: false,
            backtrack_seq_skip_item: true,
            fallback_tuple_empty: false,
            fallback_tuple_skip_item: false,
            backtrack_tuple_skip_item: true,
            fallback_tuple_struct_empty: false,
            fallback_tuple_struct_skip_item: false,
            backtrack_tuple_struct_skip_item: true,
            fallback_map_empty: false,
            fallback_map_empty_at_root: true,
            backtrack_map_empty_for_value: true,
            fallback_map_skip_item: false,
            backtrack_map_skip_item: true,
            fallback_struct_empty: false,
            fallback_struct_empty_at_root: true,
            backtrack_struct_empty_for_value: true,
            fallback_struct_skip_field: true,
            backtrack_struct_skip_field: true,
            fallback_unit_variant: true,
            fallback_other_skip_item: false,
            backtrack_other_skip_item: true,
            allow_incomplete_string_in_key_or_variant: false,
        }
    }
}

impl UnstableCustomBehavior {
    pub fn no_fallbacks(mut self) -> Self {
        let Self {
            tolerate_deserializer_fail_after_visit_success: _,
            fallback_any_as_none,
            fallback_ignored_any_as_none,
            fallback_default_bool,
            fallback_int_zero,
            fallback_default_float,
            fallback_default_char,
            fallback_default_str,
            fallback_bytes_empty,
            fallback_none,
            fallback_none_at_mandatory,
            fallback_unit,
            fallback_unit_at_mandatory,
            fallback_unit_struct,
            fallback_unit_struct_at_mandatory,
            fallback_seq_empty,
            fallback_seq_empty_at_root,
            backtrack_seq_empty_for_value: _,
            fallback_seq_skip_item,
            backtrack_seq_skip_item: _,
            fallback_tuple_empty,
            fallback_tuple_skip_item,
            backtrack_tuple_skip_item: _,
            fallback_tuple_struct_empty,
            fallback_tuple_struct_skip_item: fallback_tuple_struct_skip_field,
            backtrack_tuple_struct_skip_item: _,
            fallback_map_empty,
            fallback_map_empty_at_root,
            backtrack_map_empty_for_value: _,
            fallback_map_skip_item,
            backtrack_map_skip_item: _,
            fallback_struct_empty,
            fallback_struct_empty_at_root,
            backtrack_struct_empty_for_value: _,
            fallback_struct_skip_field,
            backtrack_struct_skip_field: _,
            fallback_unit_variant,
            fallback_other_skip_item,
            backtrack_other_skip_item: _,
            allow_incomplete_string_in_key_or_variant: allow_incomplete_string_in_key,
        } = &mut self;

        *fallback_any_as_none = false;
        *fallback_ignored_any_as_none = false;
        *fallback_default_bool = None;
        *fallback_int_zero = false;
        *fallback_default_float = None;
        *fallback_default_char = None;
        *fallback_default_str = None;
        *fallback_bytes_empty = false;
        *fallback_none = false;
        *fallback_none_at_mandatory = false;
        *fallback_unit = false;
        *fallback_unit_at_mandatory = false;
        *fallback_unit_struct = false;
        *fallback_unit_struct_at_mandatory = false;
        *fallback_seq_empty = false;
        *fallback_seq_empty_at_root = false;
        *fallback_seq_skip_item = false;
        *fallback_tuple_empty = false;
        *fallback_tuple_skip_item = false;
        *fallback_tuple_struct_empty = false;
        *fallback_tuple_struct_skip_field = false;
        *fallback_map_empty = false;
        *fallback_map_empty_at_root = false;
        *fallback_map_skip_item = false;
        *fallback_struct_empty = false;
        *fallback_struct_empty_at_root = false;
        *fallback_struct_skip_field = false;
        *fallback_unit_variant = false;
        *fallback_other_skip_item = false;
        *allow_incomplete_string_in_key = false;

        self
    }

    /// Maximally strict behavior. Probably this library behaves like ordinary
    /// deserialization with this behavior.
    pub fn strict() -> Self {
        Self {
            tolerate_deserializer_fail_after_visit_success: false,
            fallback_any_as_none: false,
            fallback_ignored_any_as_none: false,
            fallback_default_bool: None,
            fallback_int_zero: false,
            fallback_default_float: None,
            fallback_default_char: None,
            fallback_default_str: None,
            fallback_bytes_empty: false,
            fallback_none: false,
            fallback_none_at_mandatory: false,
            fallback_unit: false,
            fallback_unit_at_mandatory: false,
            fallback_unit_struct: false,
            fallback_unit_struct_at_mandatory: false,
            fallback_seq_empty: false,
            fallback_seq_empty_at_root: false,
            backtrack_seq_empty_for_value: false,
            fallback_seq_skip_item: false,
            backtrack_seq_skip_item: false,
            fallback_tuple_empty: false,
            fallback_tuple_skip_item: false,
            backtrack_tuple_skip_item: false,
            fallback_tuple_struct_skip_item: false,
            backtrack_tuple_struct_skip_item: false,
            fallback_tuple_struct_empty: false,
            fallback_map_empty: false,
            fallback_map_empty_at_root: false,
            backtrack_map_empty_for_value: false,
            fallback_map_skip_item: false,
            backtrack_map_skip_item: false,
            fallback_struct_empty: false,
            fallback_struct_empty_at_root: false,
            backtrack_struct_empty_for_value: false,
            fallback_struct_skip_field: false,
            backtrack_struct_skip_field: false,
            fallback_unit_variant: false,
            fallback_other_skip_item: false,
            backtrack_other_skip_item: false,
            allow_incomplete_string_in_key_or_variant: false,
        }
    }

    pub fn lenient() -> Self {
        Self {
            tolerate_deserializer_fail_after_visit_success: true,
            fallback_any_as_none: true,
            fallback_ignored_any_as_none: true,
            fallback_default_bool: Some(false),
            fallback_int_zero: true,
            fallback_default_float: Some(0.0),
            fallback_default_char: Some('\0'),
            fallback_default_str: Some(""),
            fallback_bytes_empty: true,
            fallback_none: true,
            fallback_none_at_mandatory: true,
            fallback_unit: true,
            fallback_unit_at_mandatory: true,
            fallback_unit_struct: true,
            fallback_unit_struct_at_mandatory: true,
            fallback_seq_empty: true,
            fallback_seq_empty_at_root: true,
            backtrack_seq_empty_for_value: true,
            fallback_seq_skip_item: true,
            backtrack_seq_skip_item: true,
            fallback_tuple_empty: true,
            fallback_tuple_skip_item: true,
            backtrack_tuple_skip_item: true,
            fallback_tuple_struct_empty: true,
            fallback_tuple_struct_skip_item: true,
            backtrack_tuple_struct_skip_item: true,
            fallback_map_empty: true,
            fallback_map_empty_at_root: true,
            backtrack_map_empty_for_value: true,
            fallback_map_skip_item: true,
            backtrack_map_skip_item: true,
            fallback_struct_empty: true,
            fallback_struct_empty_at_root: true,
            backtrack_struct_empty_for_value: true,
            fallback_struct_skip_field: true,
            backtrack_struct_skip_field: true,
            fallback_unit_variant: true,
            fallback_other_skip_item: true,
            backtrack_other_skip_item: true,
            allow_incomplete_string_in_key_or_variant: true,
        }
    }
}
