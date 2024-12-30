use std::borrow::{Borrow, BorrowMut};

use serde::Deserializer;

/// Represents the source of a data value that can be repeatedly deserialized.
/// For instance, serde_json on a borrowed string.
pub trait Source {
    /// Stack storage for the deserializer.
    type DeserializerStorage;

    /// A type that indicates how long the deserializer can be useful.
    ///
    /// Should be `&'something ()`.
    type Lifetime<'storage>;

    /// Recreate a deserializer for this source.
    ///
    /// Every deserializer created from a source should behave exactly the
    /// same.
    ///
    /// If end of file happens in a map in between the key and the value, then
    /// the first go at partially deserializing will fail, and we have to recreate
    /// a new deserializer for the same source.
    fn recreate_deserializer_storage(&mut self) -> Self::DeserializerStorage;

    /// Will be called exactly once per [`Self::DeserializerStorage`]. The argument
    /// is guaranteed to be `Some`.
    ///
    /// Typically returns either `storage.take()` or `&mut storage`.
    fn use_deserializer_from_storage<'de, 'storage>(
        storage: &'storage mut Option<Self::DeserializerStorage>,
    ) -> impl Deserializer<'de> + 'storage
    where
        Self::Lifetime<'storage>: 'de;
}

/// Use [`serde_json::from_str`].
#[cfg(feature = "serde_json")]
pub struct Json<T: Borrow<str>>(T);

/// Use [`serde_json::from_slice`].
#[cfg(feature = "serde_json")]
pub struct JsonBytes<T: Borrow<[u8]>>(T);

#[cfg(feature = "serde_json")]
impl<'de, T: Borrow<str>> Source for &'de Json<T> {
    type DeserializerStorage = serde_json::Deserializer<serde_json::de::StrRead<'de>>;

    type Lifetime<'storage> = &'de ();

    // type Deserializer<'storage> =
    //     &'storage mut serde_json::Deserializer<serde_json::de::StrRead<'de>>;

    fn recreate_deserializer_storage<'a>(&'a mut self) -> Self::DeserializerStorage {
        serde_json::Deserializer::from_str(self.0.borrow())
    }

    fn use_deserializer_from_storage<'de2, 'storage>(
        storage: &'storage mut Option<serde_json::Deserializer<serde_json::de::StrRead<'de>>>,
    ) -> &'storage mut serde_json::Deserializer<serde_json::de::StrRead<'de2>>
    where
        Self::Lifetime<'storage>: 'de2,
    {
        storage
            .as_mut()
            .expect("use_deserializer_from_storage only called on Some")
    }
}

/// Implements [`Deserializer`] when `&mut D` implements [`Deserializer`].
pub struct RefMutDeserializer<D>(pub D);

impl<'de, D> Deserializer<'de> for RefMutDeserializer<D>
where
    &'de mut D: Deserializer<'de> + 'de,
{
    type Error = <&'de mut D as Deserializer<'de>>::Error;

    fn deserialize_any<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_any(visitor)
    }

    fn deserialize_bool<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_bool(visitor)
    }

    fn deserialize_i8<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i8(visitor)
    }

    fn deserialize_i16<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i16(visitor)
    }

    fn deserialize_i32<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i32(visitor)
    }

    fn deserialize_i64<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i64(visitor)
    }

    fn deserialize_i128<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_i128(visitor)
    }

    fn deserialize_u8<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u8(visitor)
    }

    fn deserialize_u16<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u16(visitor)
    }

    fn deserialize_u32<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u32(visitor)
    }

    fn deserialize_u64<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u64(visitor)
    }

    fn deserialize_u128<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_u128(visitor)
    }

    fn deserialize_f32<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_f32(visitor)
    }

    fn deserialize_f64<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_f64(visitor)
    }

    fn deserialize_char<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_char(visitor)
    }

    fn deserialize_str<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_str(visitor)
    }

    fn deserialize_string<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_string(visitor)
    }

    fn deserialize_bytes<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_byte_buf(visitor)
    }

    fn deserialize_option<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_option(visitor)
    }

    fn deserialize_unit<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_unit(visitor)
    }

    fn deserialize_unit_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_newtype_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_seq<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_seq(visitor)
    }

    fn deserialize_tuple<V: serde::de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_map<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_map(visitor)
    }

    fn deserialize_struct<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V: serde::de::Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_enum(name, variants, visitor)
    }

    fn deserialize_identifier<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_ignored_any(visitor)
    }

    fn is_human_readable(&self) -> bool {
        self.0.is_human_readable()
    }
}
