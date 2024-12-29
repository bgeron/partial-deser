pub(crate) mod empty_access;
mod state;
mod visit;

use std::fmt::Display;

use state::AttemptState;
pub(crate) use state::GlobalState;
use visit::Visitor;

use crate::error::{Error, FallbackError};
use crate::fallback::FallbacksExt as _;
use crate::options::ExtraOptions;
use crate::reporter::{self, Reporter, ReporterExt as _};
use crate::util::{erase_error_ref, make_fnonce, DeserializeKind};

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
struct Deserializer<'a, Inner, Extra>
where
    Inner: serde::Deserializer<'a>,
    Extra: ExtraOptions,
{
    global: &'a mut GlobalState<Extra>,
    attempt: &'a mut AttemptState,
    inner: Inner,
}

fn framework<'de, InnerDeserializer, Extra, InnerVisitor>(
    inner_visitor: InnerVisitor,
    kind: DeserializeKind,
    deserializer: Deserializer<'de, InnerDeserializer, Extra>,
) -> Result<InnerVisitor::Value, Error<InnerDeserializer::Error>>
where
    Extra: ExtraOptions,
    InnerDeserializer: serde::Deserializer<'de>,
    InnerVisitor: serde::de::Visitor<'de>,
{
    let report_args = reporter::DeserializeStartArgsImpl {
        visitor: &inner_visitor,
    };
    deserializer
        .global
        .reporter
        .report_deserialize_start(report_args, kind);

    // If the deserializer actually tries to visit, then this will be consumed.
    // Otherwise we will keep it, and try to visit with a callback.
    let mut visitor = Some(inner_visitor);
    let result = deserializer.inner.deserialize_any(Visitor::new(
        deserializer.global,
        deserializer.attempt,
        &mut visitor,
    ));
    deserializer
        .global
        .reporter
        .report_deserialize_end(erase_error_ref(&result));

    if result.is_err() && visitor.is_some() {
        // We can try to apply a fallback.
        deserializer.global.reporter.report_start_fallback();
        let take_visitor =
            make_fnonce(|| visitor.take().expect("a Some can be .take()n in an FnOnce"));
        let result_opt = match deserializer.global.fallbacks.fallback(take_visitor, kind) {
            Ok(Some(value)) => Some(Ok(value)),
            Err(err) => Some(Err(FallbackError::FallbackVisitor(err))),
            Ok(None) if visitor.is_some() => None,
            Ok(None) => Some(Err(FallbackError::FallbackDidntCompute)),
        };

        if let Some(result) = result_opt {
            deserializer
                .global
                .reporter
                .report_fallback(erase_error_ref(&result));

            if let Ok(value) = result {
                return Ok(value);
            }
        } else {
            // The fallback didn't try to compute a value
            deserializer.global.reporter.report_no_fallback();
        }
    }

    result.map_err(Error::from_de)
}

impl<'de, Inner, Extra> serde::Deserializer<'de> for Deserializer<'de, Inner, Extra>
where
    Inner: serde::Deserializer<'de>,
    Extra: ExtraOptions,
{
    type Error = Error<Inner::Error>;

    fn deserialize_any<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Any, self)
    }

    fn deserialize_bool<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Bool, self)
    }

    fn deserialize_i8<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::I8, self)
    }

    fn deserialize_i16<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::I16, self)
    }

    fn deserialize_i32<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::I32, self)
    }

    fn deserialize_i64<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::I64, self)
    }

    fn deserialize_i128<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::I128, self)
    }

    fn deserialize_u8<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::U8, self)
    }

    fn deserialize_u16<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::U16, self)
    }

    fn deserialize_u32<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::U32, self)
    }

    fn deserialize_u64<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::U64, self)
    }

    fn deserialize_u128<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::U128, self)
    }

    fn deserialize_f32<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::F32, self)
    }

    fn deserialize_f64<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::F64, self)
    }

    fn deserialize_char<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Char, self)
    }

    fn deserialize_str<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Str, self)
    }

    fn deserialize_string<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::String, self)
    }

    fn deserialize_bytes<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Bytes, self)
    }

    fn deserialize_byte_buf<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::ByteBuf, self)
    }

    fn deserialize_option<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Option, self)
    }

    fn deserialize_unit<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Unit, self)
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::UnitStruct { name }, self)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::NewtypeStruct { name }, self)
    }

    fn deserialize_seq<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Seq, self)
    }

    fn deserialize_tuple<V>(self, len: usize, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Tuple { len }, self)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            inner_visitor,
            DeserializeKind::TupleStruct { name, len },
            self,
        )
    }

    fn deserialize_map<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Map, self)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            inner_visitor,
            DeserializeKind::Struct { name, fields },
            self,
        )
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            inner_visitor,
            DeserializeKind::Enum { name, variants },
            self,
        )
    }

    fn deserialize_identifier<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::Identifier, self)
    }

    fn deserialize_ignored_any<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(inner_visitor, DeserializeKind::IgnoredAny, self)
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}
