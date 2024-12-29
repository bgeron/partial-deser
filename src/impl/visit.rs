use crate::options::ExtraOptions;

use super::state::{AttemptState, GlobalState};

/// Something that creates a data value, if only you tell it what the format is like.
pub(crate) struct Visitor<'a, 'de, 'deserializer_error, Inner, Extra>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions<'deserializer_error>,
{
    pub(super) global: &'a mut GlobalState<'deserializer_error, Extra>,
    pub(super) attempt: &'a mut AttemptState,

    /// This should always be set to `Some` while the inner deserializer is being called,
    /// and thus while the [`serde::de::Visitor`] methods of [`Visitor`] are called.
    ///
    /// The inner visitor actually lives on the stack, so that in case the deserializer fails,
    /// we can attempt to apply a fallback instead.
    pub(super) inner: &'a mut Option<Inner>,
    phantom: std::marker::PhantomData<&'de ()>,
}

impl<'a,'de, 'deserializer_error, Inner, Extra> Visitor<'a, 'de, 'deserializer_error, Inner, Extra>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions<'deserializer_error>,
{
    pub(super) fn new(
        global: &'a mut GlobalState<'deserializer_error, Extra>,
        attempt: &'a mut AttemptState,
        inner_on_stack: &'a mut Option<Inner>,
    ) -> Self {
        Self {
            global,
            attempt,
            inner: inner_on_stack,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, 'de,'deserializer_error, Inner, Extra> serde::de::Visitor<'de>
    for Visitor<'a, 'de,'deserializer_error, Inner, Extra>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions<'deserializer_error>,
{
    type Value = Inner::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.as_ref().expect("the inner visitor has not been consumed while the external deserializer is running").expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Bool(v),
            &self,
        ))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        // todo
        self.visit_i64(v as i64)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        // todo
        self.visit_i64(v as i64)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        // todo
        self.visit_i64(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Signed(v),
            &self,
        ))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        todo!()
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        self.visit_u64(v as u64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        self.visit_u64(v as u64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        self.visit_u64(v as u64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Unsigned(v),
            &self,
        ))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        todo!()
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        self.visit_f64(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Float(v),
            &self,
        ))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        self.visit_str(v.encode_utf8(&mut [0u8; 4]))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Str(v),
            &self,
        ))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        self.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        self.visit_str(&v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Bytes(v),
            &self,
        ))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        self.visit_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        self.visit_bytes(&v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &self,
        ))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // todo
        let _ = deserializer;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &self,
        ))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // todo
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Unit,
            &self,
        ))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // todo
        let _ = deserializer;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::NewtypeStruct,
            &self,
        ))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        // todo
        let _ = seq;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Seq,
            &self,
        ))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        // todo
        let _ = map;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Map,
            &self,
        ))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        // todo
        let _ = data;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Enum,
            &self,
        ))
    }
}
