use serde::de::Visitor;

use crate::r#impl::empty_access::EmptyAccess;
use crate::util::DeserializeKind;

/// This describes what to do in case the data ends unexpectedly.
///
/// For instance, for a data type
///
/// ```rs
/// # use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct X {
///     i: Option<i32>,
/// }
/// ```
///
/// for JSON input `{"i": `, at end-of-input we can pretend that the JSON
/// was `{"i": null}`.
///
/// This trait specifies one case for each method of [`serde::Deserializer`].
///
/// ## Context
///
/// Fallbacks aren't crucial to the functioning of this crate.
///
/// If there is no fallback (in the example above, if [`Self::fallback_option`]
/// returns `None`), then one round of deserialization will fail but we will
/// restart and next time we know to not attempt to serialize `"i"`.`
pub trait Fallbacks {
    fn fallback_any<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_bool<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i8<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i16<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i32<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i64<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i128<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u8<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u16<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u32<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u64<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u128<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_f32<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_f64<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_char<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_str<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_string<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_bytes<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_byte_buf<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_option<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_unit<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_unit_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_newtype_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_seq<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_tuple<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_tuple_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_map<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_enum<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_identifier<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_ignored_any<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
}

pub struct DefaultFallbacks;

impl Fallbacks for DefaultFallbacks {
    fn fallback_option<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        (take_visitor)().visit_none().map(Some)
    }

    fn fallback_unit<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        (take_visitor)().visit_unit().map(Some)
    }

    fn fallback_unit_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        (take_visitor)().visit_unit().map(Some)
    }
    fn fallback_seq<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        (take_visitor)().visit_seq(EmptyAccess::default()).map(Some)
    }
    fn fallback_map<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        (take_visitor)().visit_map(EmptyAccess::default()).map(Some)
    }
}

impl<T> FallbacksExt for T where T: Fallbacks {}

/// Not public interface in the foreseeable future.
pub(crate) trait FallbacksExt: Fallbacks {
    fn fallback<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        take_visitor: impl FnOnce() -> V,
        kind: DeserializeKind,
    ) -> Result<Option<V::Value>, E> {
        match kind {
            DeserializeKind::Any => self.fallback_any(take_visitor),
            DeserializeKind::Bool => self.fallback_bool(take_visitor),
            DeserializeKind::I8 => self.fallback_i8(take_visitor),
            DeserializeKind::I16 => self.fallback_i16(take_visitor),
            DeserializeKind::I32 => self.fallback_i32(take_visitor),
            DeserializeKind::I64 => self.fallback_i64(take_visitor),
            DeserializeKind::I128 => self.fallback_i128(take_visitor),
            DeserializeKind::U8 => self.fallback_u8(take_visitor),
            DeserializeKind::U16 => self.fallback_u16(take_visitor),
            DeserializeKind::U32 => self.fallback_u32(take_visitor),
            DeserializeKind::U64 => self.fallback_u64(take_visitor),
            DeserializeKind::U128 => self.fallback_u128(take_visitor),
            DeserializeKind::F32 => self.fallback_f32(take_visitor),
            DeserializeKind::F64 => self.fallback_f64(take_visitor),
            DeserializeKind::Char => self.fallback_char(take_visitor),
            DeserializeKind::Str => self.fallback_str(take_visitor),
            DeserializeKind::String => self.fallback_string(take_visitor),
            DeserializeKind::Bytes => self.fallback_bytes(take_visitor),
            DeserializeKind::ByteBuf => self.fallback_byte_buf(take_visitor),
            DeserializeKind::Option => self.fallback_option(take_visitor),
            DeserializeKind::Unit => self.fallback_unit(take_visitor),
            DeserializeKind::UnitStruct { .. } => self.fallback_unit_struct(take_visitor),
            DeserializeKind::NewtypeStruct { .. } => self.fallback_newtype_struct(take_visitor),
            DeserializeKind::Seq => self.fallback_seq(take_visitor),
            DeserializeKind::Tuple { .. } => self.fallback_tuple(take_visitor),
            DeserializeKind::TupleStruct { .. } => self.fallback_tuple_struct(take_visitor),
            DeserializeKind::Map => self.fallback_map(take_visitor),
            DeserializeKind::Struct { .. } => self.fallback_struct(take_visitor),
            DeserializeKind::Enum { .. } => self.fallback_enum(take_visitor),
            DeserializeKind::Identifier => self.fallback_identifier(take_visitor),
            DeserializeKind::IgnoredAny => self.fallback_ignored_any(take_visitor),
        }
    }
}
