#[cfg(doc)]
use serde::de::SeqAccess;

use crate::attempt::HaltingPoint;
use crate::error::{BugEnum, InternalError};
use crate::options::ExtraOptions;
use crate::Options;

pub(crate) struct GlobalState<Extra: crate::options::ExtraOptions> {
    /// Set to 1 during first attempt
    pub(super) n_backtracks: usize,

    // technically we don't have to keep the Extra value field of Options
    pub(super) config: Options<Extra>,
    pub(super) reporter: Extra::Reporter,
    pub(super) fallbacks: Extra::FallbackProvider,
}

pub(crate) struct AttemptState {
    /// If the previous attempt failed, then there may be a point where we can tell
    /// the visitor there's no more data (e.g. in the sequence or map) and safely
    /// finish deserialization.
    pub(super) intend_to_stop_deserializing_at: Option<HaltingPoint>,

    /// Whether we have intervened in deserialization, and what the cause originally was.
    pub(super) are_intervening: Option<ReasonToIntervene>,

    pub(super) next_halting_point: HaltingPoint,
    /// Stack of points where we may halt deserialization on the next attempt.
    ///
    /// For instance, if deserializing a field failed, then on the next attempt it
    /// can make sense to halt just before that field (pretend the field is absent).
    /// But if that doesn't work then the next best thing is to halt one level up, etc.
    ///
    /// On returning an error from an attempt, this field will remain intact as of the
    /// point of the original error.
    pub(super) halting_point_stack: Vec<HaltingPoint>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum ReasonToIntervene {
    /// The deserializer has returned an error before calling the visitor.
    /// (We do not distinguish between its errors.)
    DeserializerStart,
    /// The deserializer returned an error after calling the visitor. In this
    /// case, we can salvage the
    DeserializerFinishSaved,
    /// We planned to halt deserialization at a certain point, and we have reached that point.
    PlannedHalting { at: HaltingPoint },
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
            n_backtracks: 0,
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
            next_halting_point: HaltingPoint::default(),
            halting_point_stack: Vec::new(),
        }
    }

    /// If a potential halting point was saved for next time, then create a state for
    /// that next attempt.
    ///
    /// Logs to tracing accordingly.
    pub(crate) fn fresh_state_for_next_round(mut self) -> Result<Option<Self>, InternalError> {
        match self.halting_point_stack.last().copied() {
            Some(most_recent_halting_point) => {
                if self.intend_to_stop_deserializing_at.is_some_and(
                    |intend_to_stop_deserializing_at| {
                        self.halting_point_stack
                            .iter()
                            .any(|point| **point >= *intend_to_stop_deserializing_at)
                    },
                ) {
                    return Err(BugEnum::PassedHaltingPoint {
                        intended_to_stop_at: most_recent_halting_point,
                        halting_point_stack: self.halting_point_stack,
                    }
                    .into());
                }

                trace!(
                    ?most_recent_halting_point,
                    ?self.halting_point_stack,
                    "creating state for next attempt"
                );

                self.halting_point_stack.clear();
                Ok(Some(Self {
                    intend_to_stop_deserializing_at: Some(most_recent_halting_point),
                    are_intervening: None,
                    next_halting_point: HaltingPoint::default(),
                    halting_point_stack: self.halting_point_stack,
                }))
            }
            None => {
                trace!("no halting point active after attempt, giving up");
                Ok(None)
            }
        }
    }

    pub(crate) fn get_next_halting_point(&mut self) -> HaltingPoint {
        let next = self.next_halting_point;
        self.next_halting_point.increment();
        next
    }

    /// A new halting point will now happen. Return the value of the current halting point
    /// if we're supposed to continue past this.
    ///
    /// Sets intervene state if not yet set.
    pub(crate) fn new_halting_point_and_check_continue(&mut self) -> Option<HaltingPoint> {
        let this_halting_point = self.get_next_halting_point();

        match self.intend_to_stop_deserializing_at {
            Some(stop) if *stop <= *this_halting_point => {
                if *stop < *this_halting_point {
                    error!(
                        intend_to_stop_deserializing_at=?stop,
                        ?this_halting_point,
                        "we wanted to stop at a halting point, but continued past it"
                    );
                }
                self.are_intervening
                    .get_or_insert(ReasonToIntervene::PlannedHalting { at: stop });
                None
            }
            _ => Some(this_halting_point),
        }
    }
}
