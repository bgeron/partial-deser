use crate::reporter::{DefaultReporter, Reporter};

pub struct DefaultExtraOptions;

/// Monomorphized options.
///
/// This is a type parameter pack.
pub trait ExtraOptions {
    type Reporter: crate::reporter::Reporter;

    /// Will only be called once
    fn make_reporter(&self) -> Self::Reporter;
}

impl ExtraOptions for DefaultExtraOptions {
    type Reporter = DefaultReporter;

    fn make_reporter(&self) -> Self::Reporter {
        DefaultReporter::new()
    }
}
