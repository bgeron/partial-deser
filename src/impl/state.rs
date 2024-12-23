use crate::{options::ExtraOptions, Options};

use super::Visitor;

pub(crate) struct State<Extra: crate::options::ExtraOptions> {
    pub(super) n_attempt: usize,
    // technically we don't have to keep the Extra value field of Options
    pub(super) config: Options<Extra>,
    pub(super) reporter: Extra::Reporter,
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

impl<Extra: ExtraOptions> State<Extra> {
    pub(super) fn visitor<'a, V>(&'a mut self, inner: V) -> Visitor<'a, V, Extra>
    where
        V: serde::de::Visitor<'a>,
    {
        Visitor {
            state: self,
            inner: Some(inner),
        }
    }
}
