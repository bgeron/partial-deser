use crate::attempt::AbortionPoint;
use crate::options::ExtraOptions;
use crate::Options;

pub(crate) struct GlobalState<Extra: crate::options::ExtraOptions> {
    /// Starts at 0
    pub(super) n_attempt: usize,

    pub(super) max_n_attempts: Option<usize>,

    // technically we don't have to keep the Extra value field of Options
    pub(super) config: Options<Extra>,
    pub(super) reporter: Extra::Reporter,
    pub(super) fallbacks: Extra::FallbackProvider,
}

pub(crate) struct AttemptState {
    /// If the previous attempt failed, then there may be a point where we can tell
    /// the visitor there's no more data (e.g. in the sequence or map) and safely
    /// finish deserialization.
    pub(super) intend_to_stop_deserializing_at: Option<AbortionPoint>,
    pub(super) next_abortion_point: AbortionPoint,
    /// Stack of points where we may abort deserialization on the next attempt.
    ///
    /// For instance, if deserializing a field failed, then on the next attempt it
    /// can make sense to abort just before that field (pretend the field is absent).
    /// But if that doesn't work then the next best thing is to abort one level up, etc.
    ///
    /// On returning an error from an attempt, this field will remain intact as of the
    /// point of the original error.
    pub(super) abortion_point_stack: Vec<AbortionPoint>,
}

impl<Extra: ExtraOptions> Options<Extra> {
    pub(crate) fn build(self) -> GlobalState<Extra> {
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

impl AttemptState {
    pub(crate) fn get_next_abortion_point(&mut self) -> AbortionPoint {
        let next = self.next_abortion_point;
        self.next_abortion_point.increment();
        next
    }
}
