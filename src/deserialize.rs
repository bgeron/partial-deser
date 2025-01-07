use std::marker::PhantomData;

use serde::de::DeserializeSeed;
use serde::Deserialize;

use crate::attempt::Deserializer;
use crate::error::InternalError;
use crate::options::ExtraOptions;
use crate::state::AttemptState;
use crate::{Error, Options, Source};

impl<Extra: ExtraOptions> Options<Extra> {
    /// Deserialize from a generic [`Source`].
    ///
    /// With all `deserialize_*` methods, if backtracking is needed, then
    /// execute backtracking accordingly.
    pub fn deserialize_source<'de, T, S>(self, source: S) -> Result<T, Error<S::Error>>
    where
        T: Deserialize<'de>,
        S: Source<'de>,
    {
        self.deserialize_seed(PhantomData, source)
    }

    /// Deserialize from a seed.
    pub fn deserialize_seed<'de, T, S>(
        self,
        seed: T,
        mut source: S,
    ) -> Result<T::Value, Error<S::Error>>
    where
        T: DeserializeSeed<'de> + Clone,
        S: Source<'de>,
    {
        let mut state = self.build();
        let mut attempt = AttemptState::initial();

        while {
            let max_n_backtracks = state.config.max_n_backtracks;
            max_n_backtracks.is_none_or(|max| state.n_backtracks <= max)
        } {
            let mut inner_deserializer_storage = Some(source.recreate_deserializer_storage());
            let inner_deserializer =
                S::use_deserializer_from_storage(&mut inner_deserializer_storage);

            let deserializer = Deserializer {
                global: &mut state,
                attempt: &mut attempt,
                inner: inner_deserializer,
            };

            match seed.clone().deserialize(deserializer) {
                Ok(value) => return Ok(value),
                Err(error) => {
                    debug!(attempt = state.n_backtracks, %error, "attempt failed");
                }
            }

            attempt = match attempt.fresh_state_for_next_round()? {
                Some(new_attempt) => new_attempt,
                None => {
                    return Err(InternalError::NoPotentialBacktrackPoint {
                        after_backtracks: state.n_backtracks,
                    }
                    .into())
                }
            };
            state.n_backtracks += 1;
        }

        Err(InternalError::TooManyBacktracks.into())
    }
}
