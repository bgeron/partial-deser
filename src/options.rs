use crate::{fallback::DefaultFallbacks, reporter::DefaultReporter};

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
}

/// Reexports to satisfy Rust's visibility rules TODO
#[allow(unused_imports)]
pub mod unstable {
    pub use super::{ExtraOptionsStruct, MakeFallbackProvider, MakeReporter};
    pub use crate::fallback::Fallbacks;
    pub use crate::options::UnstableCustomBehavior;
    pub use crate::reporter::Reporter;
}

pub type DefaultExtraOptions = ExtraOptionsStruct<MakeDefaultReporter, MakeDefaultFallbacks>;

#[derive(Debug, Clone, Default)]
pub struct ExtraOptionsStruct<MakeReporter, MakeFallbackProvider> {
    make_reporter: MakeReporter,
    make_fallback_provider: MakeFallbackProvider,
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

impl<R, F> ExtraOptions for ExtraOptionsStruct<R, F>
where
    R: MakeReporter,
    F: MakeFallbackProvider,
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
///     and fill in `n`n    (todo check this)
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
    unstable_fallback_any_as_none: bool,
    unstable_backtrack_any_as_none: bool,
    unstable_fallback_ignored_any_as_none: bool,
    unstable_backtrack_ignored_any_as_none: bool,
    unstable_fallback_default_bool: Option<bool>,
    unstable_backtrack_default_bool: Option<bool>,
    unstable_fallback_int_zero: bool,
    unstable_backtrack_int_zero: bool,
    unstable_fallback_default_float: Option<f32>,
    unstable_backtrack_default_float: Option<f32>,
    unstable_fallback_default_char: Option<char>,
    unstable_backtrack_default_char: Option<char>,
    unstable_fallback_default_str: Option<&'static str>,
    unstable_backtrack_default_str: Option<&'static str>,
    unstable_fallback_bytes_empty: bool,
    unstable_backtrack_bytes_empty: bool,
    unstable_fallback_none: bool,
    unstable_backtrack_none: bool,
    unstable_fallback_unit: bool,
    unstable_backtrack_unit: bool,
    unstable_fallback_unit_struct: bool,
    unstable_backtrack_unit_struct: bool,
    unstable_fallback_seq_empty: bool,
    unstable_backtrack_seq_empty: bool,
    unstable_fallback_seq_skip_item: bool,
    unstable_backtrack_seq_skip_item: bool,
    unstable_fallback_tuple_empty: bool,
    unstable_backtrack_tuple_empty: bool,
    unstable_fallback_tuple_skip_item: bool,
    unstable_backtrack_tuple_skip_item: bool,
    unstable_fallback_tuple_struct_empty: bool,
    unstable_backtrack_tuple_struct_empty: bool,
    unstable_fallback_map_empty: bool,
    unstable_backtrack_map_empty: bool,
    unstable_fallback_map_skip_item: bool,
    unstable_backtrack_map_skip_item: bool,
}

impl Default for UnstableCustomBehavior {
    fn default() -> Self {
        Self {
            unstable_fallback_any_as_none: false,
            unstable_backtrack_any_as_none: false,
            unstable_fallback_ignored_any_as_none: false,
            unstable_backtrack_ignored_any_as_none: false,
            unstable_fallback_default_bool: None,
            unstable_backtrack_default_bool: None,
            unstable_fallback_int_zero: false,
            unstable_backtrack_int_zero: false,
            unstable_fallback_default_float: None,
            unstable_backtrack_default_float: None,
            unstable_fallback_default_char: None,
            unstable_backtrack_default_char: None,
            unstable_fallback_default_str: None,
            unstable_backtrack_default_str: None,
            unstable_fallback_bytes_empty: false,
            unstable_backtrack_bytes_empty: false,
            unstable_fallback_none: true,
            unstable_backtrack_none: false,
            unstable_fallback_unit: true,
            unstable_backtrack_unit: true,
            unstable_fallback_unit_struct: true,
            unstable_backtrack_unit_struct: true,
            unstable_fallback_seq_empty: false,
            unstable_backtrack_seq_empty: false,
            unstable_fallback_seq_skip_item: true,
            unstable_backtrack_seq_skip_item: true,
            unstable_fallback_tuple_empty: false,
            unstable_backtrack_tuple_empty: false,
            unstable_fallback_tuple_skip_item: true,
            unstable_backtrack_tuple_skip_item: true,
            unstable_fallback_tuple_struct_empty: false,
            unstable_backtrack_tuple_struct_empty: false,
            unstable_fallback_map_empty: false,
            unstable_backtrack_map_empty: false,
            unstable_fallback_map_skip_item: true,
            unstable_backtrack_map_skip_item: true,
        }
    }
}

impl UnstableCustomBehavior {
    /// Maximally strict behavior. Probably this library behaves like ordinary
    /// deserialization with this behavior.
    pub fn strict() -> Self {
        Self {
            unstable_fallback_any_as_none: false,
            unstable_backtrack_any_as_none: false,
            unstable_fallback_ignored_any_as_none: false,
            unstable_backtrack_ignored_any_as_none: false,
            unstable_fallback_default_bool: None,
            unstable_backtrack_default_bool: None,
            unstable_fallback_int_zero: false,
            unstable_backtrack_int_zero: false,
            unstable_fallback_default_float: None,
            unstable_backtrack_default_float: None,
            unstable_fallback_default_char: None,
            unstable_backtrack_default_char: None,
            unstable_fallback_default_str: None,
            unstable_backtrack_default_str: None,
            unstable_fallback_bytes_empty: false,
            unstable_backtrack_bytes_empty: false,
            unstable_fallback_none: false,
            unstable_backtrack_none: false,
            unstable_fallback_unit: false,
            unstable_backtrack_unit: false,
            unstable_fallback_unit_struct: false,
            unstable_backtrack_unit_struct: false,
            unstable_fallback_seq_empty: false,
            unstable_backtrack_seq_empty: false,
            unstable_fallback_seq_skip_item: false,
            unstable_backtrack_seq_skip_item: false,
            unstable_fallback_tuple_empty: false,
            unstable_backtrack_tuple_empty: false,
            unstable_fallback_tuple_skip_item: false,
            unstable_backtrack_tuple_skip_item: false,
            unstable_fallback_tuple_struct_empty: false,
            unstable_backtrack_tuple_struct_empty: false,
            unstable_fallback_map_empty: false,
            unstable_backtrack_map_empty: false,
            unstable_fallback_map_skip_item: false,
            unstable_backtrack_map_skip_item: false,
        }
    }

    pub fn lenient() -> Self {
        Self {
            unstable_fallback_any_as_none: true,
            unstable_backtrack_any_as_none: true,
            unstable_fallback_ignored_any_as_none: true,
            unstable_backtrack_ignored_any_as_none: true,
            unstable_fallback_default_bool: Some(false),
            unstable_backtrack_default_bool: Some(false),
            unstable_fallback_int_zero: true,
            unstable_backtrack_int_zero: true,
            unstable_fallback_default_float: Some(0.0),
            unstable_backtrack_default_float: Some(0.0),
            unstable_fallback_default_char: Some('\0'),
            unstable_backtrack_default_char: Some('\0'),
            unstable_fallback_default_str: Some(""),
            unstable_backtrack_default_str: Some(""),
            unstable_fallback_bytes_empty: true,
            unstable_backtrack_bytes_empty: true,
            unstable_fallback_none: true,
            unstable_backtrack_none: true,
            unstable_fallback_unit: true,
            unstable_backtrack_unit: true,
            unstable_fallback_unit_struct: true,
            unstable_backtrack_unit_struct: true,
            unstable_fallback_seq_empty: true,
            unstable_backtrack_seq_empty: true,
            unstable_fallback_seq_skip_item: true,
            unstable_backtrack_seq_skip_item: true,
            unstable_fallback_tuple_empty: true,
            unstable_backtrack_tuple_empty: true,
            unstable_fallback_tuple_skip_item: true,
            unstable_backtrack_tuple_skip_item: true,
            unstable_fallback_tuple_struct_empty: true,
            unstable_backtrack_tuple_struct_empty: true,
            unstable_fallback_map_empty: true,
            unstable_backtrack_map_empty: true,
            unstable_fallback_map_skip_item: true,
            unstable_backtrack_map_skip_item: true,
        }
    }
}
