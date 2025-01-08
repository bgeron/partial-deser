mod access;
mod deserializer;
pub(crate) mod empty_access;
mod visit;

use std::fmt::Display;
use std::ops::Deref;

use crate::options::ExtraOptions;
use crate::state::{AttemptState, GlobalState};
use crate::util::erase_error_ref;
use crate::Error;

/// Represents a point in the deserialization process where we could choose to stop
/// deserializing and save this attempt. For instance, before a map key or before a
/// sequence element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HaltingPoint(pub(crate) u64);

impl HaltingPoint {
    pub(crate) fn increment(&mut self) {
        self.0 += 1;
    }
}

impl From<u64> for HaltingPoint {
    fn from(point: u64) -> Self {
        Self(point)
    }
}

impl Display for HaltingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "halting point {}", self.0)
    }
}

impl Deref for HaltingPoint {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// This is the deserializer with all options, including unstable interfaces.
pub(crate) struct Deserializer<'a, Inner, Extra>
where
    Extra: ExtraOptions,
{
    pub(crate) global: &'a mut GlobalState<Extra>,
    pub(crate) attempt: &'a mut AttemptState,
    pub(crate) inner: Inner,
}

pub(crate) struct DeserializeSeed<'a, Inner, Extra>
where
    Extra: ExtraOptions,
{
    pub(crate) global: &'a mut GlobalState<Extra>,
    pub(crate) attempt: &'a mut AttemptState,
    pub(crate) inner: Inner,
}

impl<'de, Inner, Extra> serde::de::DeserializeSeed<'de> for DeserializeSeed<'_, Inner, Extra>
where
    Inner: serde::de::DeserializeSeed<'de>,
    Extra: ExtraOptions,
{
    type Value = Inner::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.inner
            .deserialize(Deserializer {
                global: self.global,
                attempt: self.attempt,
                inner: deserializer,
            })
            .map_err(Error::unpack_or_make_custom)
    }
}
