use crate::{Config, DefaultOptions};

pub(crate) struct State<Options: crate::options::Options> {
    n_attempt: usize,
    config: Config,
    reporter: Options::Reporter,
}

impl Config {
    pub(crate) fn build_generic(self) -> State<DefaultOptions> {
        State {
            n_attempt: 0,
            config: self,
        }
    }
}
