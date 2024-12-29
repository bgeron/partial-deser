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

pub(crate) struct AttemptState {
    /// If the previous attempt failed, then there may be a point where we can tell
    /// the visitor there's no more data (e.g. in the sequence or map) and safely
    /// finish deserialization.
    pub(super) intend_to_stop_deserializing_at: Option<AbortionPoint>,
    pub(super) next_abortion_point: AbortionPoint,
}

impl<'deserializer_error, Extra: ExtraOptions<'deserializer_error>>
    Options<'deserializer_error, Extra>
{
    pub(crate) fn build(self) -> GlobalState<'deserializer_error, Extra> {
        let reporter = self.extra.make_reporter();
        let fallbacks = self.extra.make_fallback_provider();
        GlobalState {
            n_attempt: 0,
            config: self,
            reporter,
            fallbacks,
        }
    }
}
