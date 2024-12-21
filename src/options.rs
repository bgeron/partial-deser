use crate::reporter::DefaultReporter;

pub struct DefaultOptions;

pub trait Options {
    type Reporter: Reporter;
}

impl Options for DefaultOptions {
    type Reporter = DefaultReporter;
}
