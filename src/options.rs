use crate::{fallback::DefaultFallbacks, reporter::DefaultReporter};

pub struct DefaultExtraOptions;

/// Monomorphized options.
///
/// This is a type parameter pack.
///
/// ## Parameters
///
/// The `'error` lifetime parameter is the lifetime of deserializer errors.
/// This matters for the default reporter, which logs errors with [`tracing`],
/// and [`tracing`] only seems to accept `&(dyn std::error::Error + 'static)`. So
/// [`DefaultExtraOptions`] only implements [`ExtraOptions<'static>`].
///
/// An alternative reporter could instead always log e.g. the display representation
/// of the error. Then the corresponding parameter pack could implement [`ExtraOptions<'_>`].
pub trait ExtraOptions<'deserializer_error> {
    type Reporter: crate::reporter::Reporter<'deserializer_error>;

    /// Will only be called once
    fn make_reporter(&self) -> Self::Reporter;

    type FallbackProvider: crate::fallback::Fallbacks;
    fn make_fallback_provider(&self) -> Self::FallbackProvider;
}

impl ExtraOptions<'static> for DefaultExtraOptions {
    type Reporter = DefaultReporter;
    type FallbackProvider = DefaultFallbacks;

    fn make_reporter(&self) -> Self::Reporter {
        DefaultReporter::new()
    }

    fn make_fallback_provider(&self) -> Self::FallbackProvider {
        DefaultFallbacks
    }
}
