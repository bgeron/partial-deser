#[cfg(doc)]
use serde::de::SeqAccess;

use crate::attempt::AbortionPoint;
use crate::error::{BugEnum, InternalError};
use crate::options::ExtraOptions;
use crate::Options;

pub(crate) struct GlobalState<Extra: crate::options::ExtraOptions> {
    /// Set to 1 during first attempt
    pub(super) n_attempts: usize,

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

    /// Whether we have intervened in deserialization, and what the cause originally was.
    pub(super) are_intervening: Option<ReasonToIntervene>,

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

#[derive(Clone, Copy, Debug)]
pub(crate) enum ReasonToIntervene {
    /// The deserializer has returned an error (we do not distinguish between its errors)
    EndOfInput,
    /// We planned to abort deserialization at a certain point, and we have reached that point.
    PlannedAbortion { at: AbortionPoint },
    /// A visitor (data type) returned an error.
    ///
    /// This may well be recoverable, e.g. if the error happens inside [`SeqAccess`] or
    /// a map key.
    VisitError,
}

impl<Extra: ExtraOptions> Options<Extra> {
    pub(crate) fn build(mut self) -> GlobalState<Extra> {
        let reporter = self.extra.make_reporter();
        let fallbacks = self.extra.make_fallback_provider(&self.behavior);
        GlobalState {
            n_attempts: 0,
            config: self,
            reporter,
            fallbacks,
        }
    }
}

impl AttemptState {
    pub(crate) fn initial() -> Self {
        Self {
            intend_to_stop_deserializing_at: None,
            are_intervening: None,
            next_abortion_point: AbortionPoint::default(),
            abortion_point_stack: Vec::new(),
        }
    }

    /// If a potential abortion point was saved for next time, then create a state for
    /// that next attempt.
    ///
    /// Logs to tracing accordingly.
    pub(crate) fn fresh_state_for_next_round(mut self) -> Result<Option<Self>, InternalError> {
        match self.abortion_point_stack.last().copied() {
            Some(most_recent_abortion_point) => {
                if self.intend_to_stop_deserializing_at.is_some_and(
                    |intend_to_stop_deserializing_at| {
                        self.abortion_point_stack
                            .iter()
                            .any(|point| **point >= *intend_to_stop_deserializing_at)
                    },
                ) {
                    return Err(BugEnum::PassedAbortionPoint {
                        intended_to_stop_at: most_recent_abortion_point,
                        abortion_point_stack: self.abortion_point_stack,
                    }
                    .into());
                }

                trace!(
                    ?most_recent_abortion_point,
                    ?self.abortion_point_stack,
                    "creating state for next attempt"
                );

                self.abortion_point_stack.clear();
                Ok(Some(Self {
                    intend_to_stop_deserializing_at: Some(most_recent_abortion_point),
                    are_intervening: None,
                    next_abortion_point: AbortionPoint::default(),
                    abortion_point_stack: self.abortion_point_stack,
                }))
            }
            None => {
                trace!("no abortion point active after attempt, giving up");
                Ok(None)
            }
        }
    }

    pub(crate) fn get_next_abortion_point(&mut self) -> AbortionPoint {
        let next = self.next_abortion_point;
        self.next_abortion_point.increment();
        next
    }
}
