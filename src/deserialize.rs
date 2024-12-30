use std::marker::PhantomData;

use serde::de::DeserializeSeed;
use serde::Deserialize;

use crate::attempt::AbortionPoint;
use crate::error::{ErrorImpl, InternalError};
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
        T: DeserializeSeed<'de>,
        S: Source<'de>,
    {
        let mut state = self.build();

        // let mut attempt:AttemptState =
        while state
            .config
            .max_n_attempts
            .is_none_or(|max| state.n_attempt <= max)
        {
            let mut inner_deserializer_storage = Some(source.recreate_deserializer_storage());
            let mut inner_deserializer =
                S::use_deserializer_from_storage(&mut inner_deserializer_storage);

            // let deserializer =
            todo!()
        }

        Err(InternalError::TooManyBacktracks.into())
    }
}
