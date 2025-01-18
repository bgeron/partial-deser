#[cfg(doc)]
use serde::de::{Deserializer, EnumAccess, MapAccess};

use crate::fallback::DefaultFallbacks;
pub use crate::random_trailer::RandomTrailer;
use crate::random_trailer::{NoopRandomTrailer, StringLike};
use crate::reporter::DefaultReporter;
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
pub trait ExtraOptions {
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
    pub unstable_tolerate_deserializer_fail_after_visit_success: bool,
    pub unstable_fallback_any_as_none: bool,
    // pub unstable_backtrack_any_as_none: bool,
    pub unstable_fallback_ignored_any_as_none: bool,
    // pub unstable_backtrack_ignored_any_as_none: bool,
    pub unstable_fallback_default_bool: Option<bool>,
    // pub unstable_backtrack_default_bool: Option<bool>,
    pub unstable_fallback_int_zero: bool,
    // pub unstable_backtrack_int_zero: bool,
    pub unstable_fallback_default_float: Option<f32>,
    // pub unstable_backtrack_default_float: Option<f32>,
    pub unstable_fallback_default_char: Option<char>,
    // pub unstable_backtrack_default_char: Option<char>,
    pub unstable_fallback_default_str: Option<&'static str>,
    // pub unstable_backtrack_default_str: Option<&'static str>,
    pub unstable_fallback_bytes_empty: bool,
    // pub unstable_backtrack_bytes_empty: bool,
    /// In case [`Deserializer::deserialize_option`] does not do anything, then
    /// just go in and visit a None.
    pub unstable_fallback_none: bool,
    pub unstable_fallback_none_at_root: bool,
    /// In case [`Deserializer::deserialize_unit`] does not do anything, then
    /// just go in and visit a unit.
    ///
    /// ## Caveat
    ///
    /// In combination with the JSON string trick, this can sometimes generate spurious
    /// list elements.
    pub unstable_fallback_unit: bool,
    pub unstable_fallback_unit_at_root: bool,

    /// In case [`Deserializer::deserialize_unit_struct`] does not do anything, then
    /// just go in and visit a unit struct.
    ///
    /// ## Caveat
    ///
    /// In combination with the JSON string trick, this can sometimes generate spurious
    /// list elements.
    pub unstable_fallback_unit_struct: bool,
    pub unstable_fallback_unit_struct_at_root: bool,

    // pub unstable_backtrack_unit_struct: bool,
    pub unstable_fallback_seq_empty: bool,
    pub unstable_fallback_seq_empty_at_root: bool,
    // pub unstable_backtrack_seq_empty: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub unstable_fallback_seq_skip_item: bool,
    pub unstable_backtrack_seq_skip_item: bool,
    pub unstable_fallback_tuple_empty: bool,
    // pub unstable_backtrack_tuple_empty: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub unstable_fallback_tuple_skip_item: bool,
    pub unstable_backtrack_tuple_skip_item: bool,
    pub unstable_fallback_tuple_struct_empty: bool,
    // pub unstable_backtrack_tuple_struct_empty: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub unstable_fallback_tuple_struct_skip_item: bool,
    pub unstable_backtrack_tuple_struct_skip_item: bool,
    pub unstable_fallback_map_empty: bool,
    pub unstable_fallback_map_empty_at_root: bool,
    // pub unstable_backtrack_map_empty: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub unstable_fallback_map_skip_item: bool,
    pub unstable_backtrack_map_skip_item: bool,
    pub unstable_fallback_struct_empty: bool,
    pub unstable_fallback_struct_empty_at_root: bool,
    // pub unstable_backtrack_struct_empty: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub unstable_fallback_struct_skip_field: bool,
    pub unstable_backtrack_struct_skip_field: bool,
    pub unstable_fallback_unit_variant: bool,

    /// Whether it's okay to fallback to skipping an element or field in circumstances
    /// other than [`Deserializer::deserialize_seq`] or [`Deserializer::deserialize_tuple`].
    ///
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub unstable_fallback_other_skip_item: bool,
    pub unstable_backtrack_other_skip_item: bool,

    /// Whether incomplete strings should be allowed or rejected
    /// in [`MapAccess::next_key`] or [`EnumAccess::variant`].
    pub unstable_allow_incomplete_string_in_key_or_variant: bool,
}

impl Default for UnstableCustomBehavior {
    fn default() -> Self {
        Self {
            unstable_tolerate_deserializer_fail_after_visit_success: true,
            unstable_fallback_any_as_none: false,
            // unstable_backtrack_any_as_none: false,
            unstable_fallback_ignored_any_as_none: false,
            // unstable_backtrack_ignored_any_as_none: false,
            unstable_fallback_default_bool: None,
            // unstable_backtrack_default_bool: None,
            unstable_fallback_int_zero: false,
            // unstable_backtrack_int_zero: false,
            unstable_fallback_default_float: None,
            // unstable_backtrack_default_float: None,
            unstable_fallback_default_char: None,
            // unstable_backtrack_default_char: None,
            unstable_fallback_default_str: None,
            // unstable_backtrack_default_str: None,
            unstable_fallback_bytes_empty: false,
            // unstable_backtrack_bytes_empty: false,
            unstable_fallback_none: true,
            unstable_fallback_none_at_root: true,
            unstable_fallback_unit: false,
            unstable_fallback_unit_at_root: true,
            // unstable_backtrack_unit: true,
            unstable_fallback_unit_struct: false,
            unstable_fallback_unit_struct_at_root: true,
            // unstable_backtrack_unit_struct: true,
            unstable_fallback_seq_empty: false,
            unstable_fallback_seq_empty_at_root: true,
            // unstable_backtrack_seq_empty: false,
            unstable_fallback_seq_skip_item: false,
            unstable_backtrack_seq_skip_item: true,
            unstable_fallback_tuple_empty: false,
            // unstable_backtrack_tuple_empty: false,
            unstable_fallback_tuple_skip_item: false,
            unstable_backtrack_tuple_skip_item: true,
            unstable_fallback_tuple_struct_empty: false,
            // unstable_backtrack_tuple_struct_empty: false,
            unstable_fallback_tuple_struct_skip_item: false,
            unstable_backtrack_tuple_struct_skip_item: true,
            unstable_fallback_map_empty: false,
            unstable_fallback_map_empty_at_root: true,
            // unstable_backtrack_map_empty: false,
            unstable_fallback_map_skip_item: false,
            unstable_backtrack_map_skip_item: true,
            unstable_fallback_struct_empty: false,
            unstable_fallback_struct_empty_at_root: true,
            unstable_fallback_struct_skip_field: true,
            unstable_backtrack_struct_skip_field: true,
            unstable_fallback_unit_variant: true,
            unstable_fallback_other_skip_item: false,
            unstable_backtrack_other_skip_item: true,
            unstable_allow_incomplete_string_in_key_or_variant: false,
        }
    }
}

impl UnstableCustomBehavior {
    pub fn no_fallbacks(mut self) -> Self {
        let Self {
            unstable_tolerate_deserializer_fail_after_visit_success: _,
            unstable_fallback_any_as_none,
            unstable_fallback_ignored_any_as_none,
            unstable_fallback_default_bool,
            unstable_fallback_int_zero,
            unstable_fallback_default_float,
            unstable_fallback_default_char,
            unstable_fallback_default_str,
            unstable_fallback_bytes_empty,
            unstable_fallback_none,
            unstable_fallback_none_at_root,
            unstable_fallback_unit,
            unstable_fallback_unit_at_root,
            unstable_fallback_unit_struct,
            unstable_fallback_unit_struct_at_root,
            unstable_fallback_seq_empty,
            unstable_fallback_seq_empty_at_root,
            unstable_fallback_seq_skip_item,
            unstable_backtrack_seq_skip_item: _,
            unstable_fallback_tuple_empty,
            unstable_fallback_tuple_skip_item,
            unstable_backtrack_tuple_skip_item: _,
            unstable_fallback_tuple_struct_empty,
            unstable_fallback_tuple_struct_skip_item: unstable_fallback_tuple_struct_skip_field,
            unstable_backtrack_tuple_struct_skip_item: _,
            unstable_fallback_map_empty,
            unstable_fallback_map_empty_at_root,
            unstable_fallback_map_skip_item,
            unstable_backtrack_map_skip_item: _,
            unstable_fallback_struct_empty,
            unstable_fallback_struct_empty_at_root,
            unstable_fallback_struct_skip_field,
            unstable_backtrack_struct_skip_field: _,
            unstable_fallback_unit_variant,
            unstable_fallback_other_skip_item,
            unstable_backtrack_other_skip_item: _,
            unstable_allow_incomplete_string_in_key_or_variant:
                unstable_allow_incomplete_string_in_key,
        } = &mut self;

        *unstable_fallback_any_as_none = false;
        *unstable_fallback_ignored_any_as_none = false;
        *unstable_fallback_default_bool = None;
        *unstable_fallback_int_zero = false;
        *unstable_fallback_default_float = None;
        *unstable_fallback_default_char = None;
        *unstable_fallback_default_str = None;
        *unstable_fallback_bytes_empty = false;
        *unstable_fallback_none = false;
        *unstable_fallback_none_at_root = false;
        *unstable_fallback_unit = false;
        *unstable_fallback_unit_at_root = false;
        *unstable_fallback_unit_struct = false;
        *unstable_fallback_unit_struct_at_root = false;
        *unstable_fallback_seq_empty = false;
        *unstable_fallback_seq_empty_at_root = false;
        *unstable_fallback_seq_skip_item = false;
        *unstable_fallback_tuple_empty = false;
        *unstable_fallback_tuple_skip_item = false;
        *unstable_fallback_tuple_struct_empty = false;
        *unstable_fallback_tuple_struct_skip_field = false;
        *unstable_fallback_map_empty = false;
        *unstable_fallback_map_empty_at_root = false;
        *unstable_fallback_map_skip_item = false;
        *unstable_fallback_struct_empty = false;
        *unstable_fallback_struct_empty_at_root = false;
        *unstable_fallback_struct_skip_field = false;
        *unstable_fallback_unit_variant = false;
        *unstable_fallback_other_skip_item = false;
        *unstable_allow_incomplete_string_in_key = false;

        self
    }

    /// Maximally strict behavior. Probably this library behaves like ordinary
    /// deserialization with this behavior.
    pub fn strict() -> Self {
        Self {
            unstable_tolerate_deserializer_fail_after_visit_success: false,
            unstable_fallback_any_as_none: false,
            // unstable_backtrack_any_as_none: false,
            unstable_fallback_ignored_any_as_none: false,
            // unstable_backtrack_ignored_any_as_none: false,
            unstable_fallback_default_bool: None,
            // unstable_backtrack_default_bool: None,
            unstable_fallback_int_zero: false,
            // unstable_backtrack_int_zero: false,
            unstable_fallback_default_float: None,
            // unstable_backtrack_default_float: None,
            unstable_fallback_default_char: None,
            // unstable_backtrack_default_char: None,
            unstable_fallback_default_str: None,
            // unstable_backtrack_default_str: None,
            unstable_fallback_bytes_empty: false,
            // unstable_backtrack_bytes_empty: false,
            unstable_fallback_none: false,
            unstable_fallback_none_at_root: false,
            unstable_fallback_unit: false,
            unstable_fallback_unit_at_root: false,
            // unstable_backtrack_unit: false,
            unstable_fallback_unit_struct: false,
            unstable_fallback_unit_struct_at_root: false,
            // unstable_backtrack_unit_struct: false,
            unstable_fallback_seq_empty: false,
            unstable_fallback_seq_empty_at_root: false,
            // unstable_backtrack_seq_empty: false,
            unstable_fallback_seq_skip_item: false,
            unstable_backtrack_seq_skip_item: false,
            unstable_fallback_tuple_empty: false,
            // unstable_backtrack_tuple_empty: false,
            unstable_fallback_tuple_skip_item: false,
            unstable_backtrack_tuple_skip_item: false,
            unstable_fallback_tuple_struct_skip_item: false,
            unstable_backtrack_tuple_struct_skip_item: false,
            unstable_fallback_tuple_struct_empty: false,
            // unstable_backtrack_tuple_struct_empty: false,
            unstable_fallback_map_empty: false,
            unstable_fallback_map_empty_at_root: false,
            // unstable_backtrack_map_empty: false,
            unstable_fallback_map_skip_item: false,
            unstable_backtrack_map_skip_item: false,
            unstable_fallback_struct_empty: false,
            unstable_fallback_struct_empty_at_root: false,
            // unstable_backtrack_struct_empty: false,
            unstable_fallback_struct_skip_field: false,
            unstable_backtrack_struct_skip_field: false,
            unstable_fallback_unit_variant: false,
            unstable_fallback_other_skip_item: false,
            unstable_backtrack_other_skip_item: false,
            unstable_allow_incomplete_string_in_key_or_variant: false,
        }
    }

    pub fn lenient() -> Self {
        Self {
            unstable_tolerate_deserializer_fail_after_visit_success: true,
            unstable_fallback_any_as_none: true,
            // unstable_backtrack_any_as_none: true,
            unstable_fallback_ignored_any_as_none: true,
            // unstable_backtrack_ignored_any_as_none: true,
            unstable_fallback_default_bool: Some(false),
            // unstable_backtrack_default_bool: Some(false),
            unstable_fallback_int_zero: true,
            // unstable_backtrack_int_zero: true,
            unstable_fallback_default_float: Some(0.0),
            // unstable_backtrack_default_float: Some(0.0),
            unstable_fallback_default_char: Some('\0'),
            // unstable_backtrack_default_char: Some('\0'),
            unstable_fallback_default_str: Some(""),
            // unstable_backtrack_default_str: Some(""),
            unstable_fallback_bytes_empty: true,
            // unstable_backtrack_bytes_empty: true,
            unstable_fallback_none: true,
            unstable_fallback_none_at_root: true,
            unstable_fallback_unit: true,
            unstable_fallback_unit_at_root: true,
            // unstable_backtrack_unit: true,
            unstable_fallback_unit_struct: true,
            unstable_fallback_unit_struct_at_root: true,
            // unstable_backtrack_unit_struct: true,
            unstable_fallback_seq_empty: true,
            unstable_fallback_seq_empty_at_root: true,
            // unstable_backtrack_seq_empty: true,
            unstable_fallback_seq_skip_item: true,
            unstable_backtrack_seq_skip_item: true,
            unstable_fallback_tuple_empty: true,
            // unstable_backtrack_tuple_empty: true,
            unstable_fallback_tuple_skip_item: true,
            unstable_backtrack_tuple_skip_item: true,
            unstable_fallback_tuple_struct_empty: true,
            // unstable_backtrack_tuple_struct_empty: true,
            unstable_fallback_tuple_struct_skip_item: true,
            unstable_backtrack_tuple_struct_skip_item: true,
            unstable_fallback_map_empty: true,
            unstable_fallback_map_empty_at_root: true,
            // unstable_backtrack_map_empty: true,
            unstable_fallback_map_skip_item: true,
            unstable_backtrack_map_skip_item: true,
            unstable_fallback_struct_empty: true,
            unstable_fallback_struct_empty_at_root: true,
            // unstable_backtrack_struct_empty: true,
            unstable_fallback_struct_skip_field: true,
            unstable_backtrack_struct_skip_field: true,
            unstable_fallback_unit_variant: true,
            unstable_fallback_other_skip_item: true,
            unstable_backtrack_other_skip_item: true,
            unstable_allow_incomplete_string_in_key_or_variant: true,
        }
    }
}
