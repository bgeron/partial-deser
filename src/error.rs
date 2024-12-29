use std::fmt::Display;

use serde::de::{Expected, Unexpected};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error<DeserializerErr> {
    // We always wrap in Box, mimicing serde_json. This is supposed to
    // make the return types of lots of intermediate functions smaller
    // and therefore faster. I don't know if this also matters for
    // partial-deser.
    err: Box<ErrorImpl<DeserializerErr>>,
}

#[derive(Debug, thiserror::Error)]
enum ErrorImpl<DeserializerErr> {
    /// The wrapped deserializer returned an error.
    ///
    /// (todo: keep? I guess most deserializer errors in practice are sort of equivalent to EOF?)
    #[error(transparent)]
    Deserializer(DeserializerErr),
    /// The deserializer behaved in an inconsistent / nondeterministic way.
    #[error(transparent)]
    InconsistentDeserializer(InconsistentDeserializerErr),
    /// One of the [`crate::fallback::Fallbacks`] failed to compute.
    ///
    /// The concrete error type within is not stable.
    #[error(transparent)]
    Fallback(FallbackError),
}

#[derive(Debug, thiserror::Error)]
pub enum InconsistentDeserializerErr {}

#[derive(Debug, thiserror::Error)]
pub enum FallbackError {
    #[error("The fallback took the visitor to compute a value, but didn't return a Result.")]
    FallbackDidntCompute,
    #[error("While constructing a fallback value: {0}")]
    FallbackVisitor(serde::de::value::Error),
}

impl<DeserializerErr> Error<DeserializerErr> {
    pub(crate) fn from_de(err: DeserializerErr) -> Self {
        Self {
            err: Box::new(ErrorImpl::Deserializer(err)),
        }
    }

    /// Was it an error from the wrapped deserializer?
    pub fn as_deserializer_error(&self) -> Option<&DeserializerErr> {
        match &*self.err {
            ErrorImpl::Deserializer(err) => Some(err),
            _ => None,
        }
    }

    pub fn into_deserializer_error(self) -> Option<DeserializerErr> {
        match *self.err {
            ErrorImpl::Deserializer(err) => Some(err),
            _ => None,
        }
    }

    /// Was the deserializer being inconsistent?
    pub fn as_inconsistent_deserializer_error(&self) -> Option<&InconsistentDeserializerErr> {
        match &*self.err {
            ErrorImpl::InconsistentDeserializer(err) => Some(err),
            _ => None,
        }
    }

    pub fn into_inconsistent_deserializer_error(self) -> Option<InconsistentDeserializerErr> {
        match *self.err {
            ErrorImpl::InconsistentDeserializer(err) => Some(err),
            _ => None,
        }
    }

    /// Did we try to construct a fallback error?
    pub fn is_fallback_error(&self) -> bool {
        matches!(&*self.err, ErrorImpl::Fallback(_))
    }
}

impl<DeserializerErr> From<InconsistentDeserializerErr> for Error<DeserializerErr> {
    fn from(err: InconsistentDeserializerErr) -> Self {
        Self {
            err: Box::new(ErrorImpl::InconsistentDeserializer(err)),
        }
    }
}

impl<DeserializerErr> serde::de::Error for Error<DeserializerErr>
where
    DeserializerErr: serde::de::Error,
{
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::from_de(DeserializerErr::custom(msg))
    }

    #[cold]
    fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Self::from_de(DeserializerErr::invalid_type(unexp, exp))
    }

    #[cold]
    fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Self::from_de(DeserializerErr::invalid_value(unexp, exp))
    }

    #[cold]
    fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
        Self::from_de(DeserializerErr::invalid_length(len, exp))
    }

    #[cold]
    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Self::from_de(DeserializerErr::unknown_variant(variant, expected))
    }

    #[cold]
    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Self::from_de(DeserializerErr::unknown_field(field, expected))
    }

    #[cold]
    fn missing_field(field: &'static str) -> Self {
        Self::from_de(DeserializerErr::missing_field(field))
    }

    #[cold]
    fn duplicate_field(field: &'static str) -> Self {
        Self::from_de(DeserializerErr::duplicate_field(field))
    }
}
