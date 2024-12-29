use serde::de::Visitor;

use crate::r#impl::empty_access::EmptyAccess;

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
