use crate::{fallback::DefaultFallbacks, reporter::DefaultReporter};

pub struct DefaultExtraOptions;

/// Monomorphized options.
///
/// This is a type parameter pack.
pub trait ExtraOptions {
    type Reporter: crate::reporter::Reporter;

    /// Will only be called once
    fn make_reporter(&self) -> Self::Reporter;

    type FallbackProvider: crate::fallback::Fallbacks;
    fn make_fallback_provider(&self) -> Self::FallbackProvider;
}

impl ExtraOptions for DefaultExtraOptions {
    type Reporter = DefaultReporter;
    type FallbackProvider = DefaultFallbacks;

    fn make_reporter(&self) -> Self::Reporter {
        DefaultReporter::new()
    }

    fn make_fallback_provider(&self) -> Self::FallbackProvider {
        DefaultFallbacks
    }
}
