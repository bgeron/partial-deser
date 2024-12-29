pub(crate) mod empty_access;
mod state;
mod visit;

use std::fmt::Display;
use std::marker::PhantomData;

use state::AttemptState;
pub(crate) use state::GlobalState;
use visit::Visitor;

use crate::error::{Error, FallbackError};
use crate::fallback::Fallbacks;
use crate::options::ExtraOptions;
use crate::reporter::{self, Reporter};
use crate::util::{erase_error_ref, make_fnonce};

/// Represents a point in the deserialization process where we could choose to stop
/// deserializing and save this attempt. For instance, before a map key or before a
/// sequence element.
#[derive(Clone, Copy, Debug)]
struct AbortionPoint(pub usize);

impl AbortionPoint {
    fn new(point: usize) -> Self {
        Self(point)
    }

    fn increment(&mut self) {
        self.0 += 1;
    }
}

impl From<usize> for AbortionPoint {
    fn from(point: usize) -> Self {
        Self(point)
    }
}

impl Display for AbortionPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "abortion point {}", self.0)
    }
}

/// This is the deserializer with all options, including unstable interfaces.
struct Deserializer<'a, 'deserializer_error, Inner, Extra>
where
    Inner: serde::Deserializer<'a>,
    Extra: ExtraOptions<'deserializer_error>,
{
    global: &'a mut GlobalState<'deserializer_error, Extra>,
    attempt: &'a mut AttemptState,
    inner: Inner,
    phantom: PhantomData<&'deserializer_error ()>,
}

impl<'de, 'deserializer_error, Inner, Extra> serde::Deserializer<'de>
    for Deserializer<'de, 'deserializer_error, Inner, Extra>
where
    Inner: serde::Deserializer<'de>,
    Inner::Error: 'deserializer_error,
    Extra: ExtraOptions<'deserializer_error>,
{
    type Error = Error<Inner::Error>;

    fn deserialize_any<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        {
            self.global.reporter.report_deserialize_start_any(
                &(reporter::DeserializeBeginArgsImpl {
                    visitor: &inner_visitor,
                }),
            );
        }
        let mut visitor = Some(inner_visitor);
        let result;
        {
            let wrapped =
                Visitor::<'_, 'de, '_, V, _>::new(self.global, self.attempt, &mut visitor);
            result = self.inner.deserialize_any(wrapped);
        }
        self.global
            .reporter
            .report_deserialize_end(erase_error_ref(&result));

        #[cold]
        if result.is_err() && visitor.is_some() {
            // We can try to apply a fallback.
            self.global.reporter.report_start_fallback();
            let take_visitor =
                make_fnonce(|| visitor.take().expect("a Some can be .take()n in an FnOnce"));
            let result_opt = match self.global.fallbacks.fallback_any(take_visitor) {
                Ok(Some(value)) => Some(Ok(value)),
                Err(err) => Some(Err(FallbackError::FallbackVisitor(err))),
                Ok(None) if visitor.is_some() => None,
                Ok(None) => Some(Err(FallbackError::FallbackDidntCompute)),
            };

            if let Some(result) = result_opt {
                self.global
                    .reporter
                    .report_fallback(erase_error_ref(&result));

                if let Ok(value) = result {
                    return Ok(value);
                }
            } else {
                // The fallback didn't try to compute a value
                self.global.reporter.report_no_fallback();
            }
        }

        result.map_err(Error::from_de)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
