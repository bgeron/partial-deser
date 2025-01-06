use std::marker::PhantomData;

use serde::de::SeqAccess;

use crate::{options::ExtraOptions, Error};

use super::{AttemptState, GlobalState};

pub(crate) struct Access<'a, 'de, Inner, Extra>
where
    Extra: ExtraOptions,
{
    pub(crate) global: &'a mut GlobalState<Extra>,
    pub(crate) attempt: &'a mut AttemptState,
    pub(crate) inner: Inner,
    pub(crate) phantom: PhantomData<fn(&'de ())>,
}

// impl<'a, 'de, Inner, Extra> SeqAccess for Access<'a, 'de, Inner, Extra>
// where
//     Inner: SeqAccess<'de>,
// {
//     type Error = Error<Inner::Error>;

//     fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
//     where
//         T: DeserializeSeed<'de>,
//     {
//         self.inner.next_element_seed(seed)
//     }
// }
