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
    #[error(transparent)]
    Deserializer(DeserializerErr),
    /// The
    #[error(transparent)]
    InvalidFormat(InvalidFormatErr),
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidFormatErr {}

impl<DeserializerErr> Error<DeserializerErr> {
    fn from_de(err: DeserializerErr) -> Self {
        Self {
            err: Box::new(ErrorImpl::Deserializer(err)),
        }
    }

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

    pub fn as_invalid_format_error(&self) -> Option<&InvalidFormatErr> {
        match &*self.err {
            ErrorImpl::InvalidFormat(err) => Some(err),
            _ => None,
        }
    }

    pub fn into_invalid_format_error(self) -> Option<InvalidFormatErr> {
        match *self.err {
            ErrorImpl::InvalidFormat(err) => Some(err),
            _ => None,
        }
    }
}

impl<DeserializerErr> From<InvalidFormatErr> for Error<DeserializerErr> {
    fn from(err: InvalidFormatErr) -> Self {
        Self {
            err: Box::new(ErrorImpl::InvalidFormat(err)),
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
