use crate::{options::ExtraOptions, Options};

use super::{AbortionPoint, Visitor};

pub(crate) struct GlobalState<
    'deserializer_error,
    Extra: crate::options::ExtraOptions<'deserializer_error>,
> {
    /// Starts at 0
    pub(super) n_attempt: usize,

    // technically we don't have to keep the Extra value field of Options
    pub(super) config: Options<'deserializer_error, Extra>,
    pub(super) reporter: Extra::Reporter,
    pub(super) fallbacks: Extra::FallbackProvider,
}

pub(crate) struct AttemptState<
    'global,
    'deserializer_error,
    Extra: ExtraOptions<'deserializer_error>,
> {
    /// If the previous attempt failed, then there may be a point where we can tell
    /// the visitor there's no more data (e.g. in the sequence or map) and safely
    /// finish deserialization.
    pub(super) intend_to_stop_deserializing_at: Option<AbortionPoint>,
    pub(super) next_abortion_point: AbortionPoint,
}

impl<'a, 'deserializer_error, Extra: ExtraOptions<'deserializer_error>> std::ops::Deref
    for AttemptState<'a, 'deserializer_error, Extra>
{
    type Target = GlobalState<'deserializer_error, Extra>;

    fn deref(&self) -> &Self::Target {
        &self.global
    }
}

impl<'a, 'deserializer_error, Extra: ExtraOptions<'deserializer_error>> std::ops::DerefMut
    for AttemptState<'a, 'deserializer_error, Extra>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global
    }
}

impl<'deserializer_error, Extra: ExtraOptions<'deserializer_error>>
    Options<'deserializer_error, Extra>
{
    pub(crate) fn build(self) -> GlobalState<'deserializer_error, Extra> {
        let reporter = self.extra.make_reporter();
        GlobalState {
            n_attempt: 0,
            config: self,
            reporter,
        }
    }
}

impl<'a, 'visitor, 'deserializer_error, Extra: ExtraOptions<'deserializer_error>>
    AttemptState<'a, 'deserializer_error, Extra>
{
    pub(super) fn visitor<V>(
        &'a mut self,
        inner_on_stack: &'a mut Option<V>,
    ) -> Visitor<'visitor, 'deserializer_error, V, Extra>
    where
        V: serde::de::Visitor<'visitor>,
        'visitor: 'a,
    {
        Visitor {
            state: self,
            inner: inner_on_stack,
        }
    }
}
