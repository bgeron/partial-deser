use crate::{options::ExtraOptions,  Options};

pub(crate) struct State<Extra: crate::options::ExtraOptions> {
    n_attempt: usize,
    // technically we don't have to keep the Extra value field of Options
    config: Options<Extra>,
    reporter: Extra::Reporter,
}

impl<Extra: ExtraOptions> Options<Extra> {
    pub(crate) fn build(self) -> State<Extra> {
        let reporter = self.extra.make_reporter();
        State {
            n_attempt: 0,
            config: self,
            reporter,
        }
    }
}
