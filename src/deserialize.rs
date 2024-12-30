use serde::Deserialize;

use crate::options::ExtraOptions;
use crate::{Error, Options, Source};

impl<Extra: ExtraOptions> Options<Extra> {
    /// Deserialize from a generic [`Source`].
    /// 
    /// With all `deserialize_*` methods, if backtracking is needed, then
    /// execute backtracking accordingly.
    /// 
    /// This 
    pub(super) fn deserialize_source<'de, T, S>(
        &self,
        source: S,
    ) -> Result<T, Error<<S::Deserializer as serde::Deserializer<'de>>::Error>>
    where
        T: Deserialize<'de>,
        S: Source<'de>,
    {
        todo!()
    }


    /// Deserialize from a seed.
}

// deserialize_seed
