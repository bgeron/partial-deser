use crate::Options;



pub(crate) struct State<Source, Reporter> {
    n_attempt: usize,
    options: Options,
    reporter:
}

impl Options {
    pub(crate) fn build_generic(self, ) -> State {
        State {
            n_attempt: 0,
            options: self.clone(),
        }
    }
}