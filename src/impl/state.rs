use crate::{options::ExtraOptions, Options};

use super::{AbortionPoint, Visitor};

pub(crate) struct State<Extra: crate::options::ExtraOptions> {
    /// Starts at 0
    pub(super) n_attempt: usize,
    /// If the previous attempt failed, then there may be a point where we can tell
    /// the visitor there's no more data (e.g. in the sequence or map) and safely
    /// finish deserialization.
    pub(super) intend_to_stop_deserializing_at: Option<AbortionPoint>,
    /// While going through an attempt, this counts what abortion point we're at.
    pub(super) next_abortion_point_in_attempt: AbortionPoint,

    // technically we don't have to keep the Extra value field of Options
    pub(super) config: Options<Extra>,
    pub(super) reporter: Extra::Reporter,
}

impl<Extra: ExtraOptions> Options<Extra> {
    pub(crate) fn build(self) -> State<Extra> {
        let reporter = self.extra.make_reporter();
        State {
            n_attempt: 0,
            intend_to_stop_deserializing_at: None,
            config: self,
            reporter,
        }
    }
}

impl<Extra: ExtraOptions> State<Extra> {
    pub(super) fn visitor<'a, V>(
        &'a mut self,
        inner_on_stack: &mut Option<V>,
    ) -> Visitor<'a, V, Extra>
    where
        V: serde::de::Visitor<'a>,
    {
        Visitor {
            state: self,
            inner: inner_on_stack,
        }
    }
}
