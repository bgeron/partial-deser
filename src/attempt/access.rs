use serde::de::SeqAccess;

use crate::{options::ExtraOptions, Error};

use super::{AttemptState, DeserializeSeed, GlobalState};

pub(crate) struct Access<'a, Inner, Extra>
where
    Extra: ExtraOptions,
{
    pub(crate) global: &'a mut GlobalState<Extra>,
    pub(crate) attempt: &'a mut AttemptState,
    pub(crate) inner: Inner,
}

impl< 'de, Inner, Extra> SeqAccess <'de> for Access<'_, Inner, Extra>
where
    Inner: SeqAccess<'de>,
    Extra: ExtraOptions,
{
    type Error = Error<Inner::Error>;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.inner.next_element_seed(DeserializeSeed {
            global: self.global,
            attempt: self.attempt,
            inner: seed,
        }).map_err(Error::from_de)
    }
}
