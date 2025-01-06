use std::error::Error as StdError;

use tap::Tap;

use super::erase_error_ref;
use crate::options::ExtraOptions;
use crate::reporter::Reporter;
use crate::state::{AttemptState, GlobalState};

/// Something that creates a data value, if only you tell it what the format is like.
pub(crate) struct Visitor<'a, 'de, Inner, Extra>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions,
{
    pub(super) global: &'a mut GlobalState<Extra>,
    pub(super) attempt: &'a mut AttemptState,

    /// This should always be set to `Some` while the inner deserializer is being called,
    /// and thus while the [`serde::de::Visitor`] methods of [`Visitor`] are called.
    ///
    /// The inner visitor actually lives on the stack, so that in case the deserializer fails,
    /// we can attempt to apply a fallback instead.
    pub(super) inner: &'a mut Option<Inner>,
    phantom: std::marker::PhantomData<&'de ()>,
}

impl<'a, 'de, Inner, Extra> Visitor<'a, 'de, Inner, Extra>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions,
{
    pub(super) fn new(
        global: &'a mut GlobalState<Extra>,
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

fn framework<'de, Inner, Extra, E>(
    visitor: Visitor<'_, 'de, Inner, Extra>,
    do_visit: impl FnOnce(Inner) -> Result<Inner::Value, E>,
    report_end: impl FnOnce(&mut Extra::Reporter, Option<&dyn StdError>),
) -> Result<Inner::Value, E>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions,
    E: serde::de::Error,
{
    let inner_visitor = visitor
        .inner
        .take()
        .expect("inner visitor is present when running Visitor");

    do_visit(inner_visitor).tap(|result| {
        visitor
            .attempt
            .are_intervening
            .get_or_insert(crate::state::ReasonToIntervene::VisitError);
        report_end(&mut visitor.global.reporter, erase_error_ref(result))
    })
}

impl<'de, Inner, Extra> serde::de::Visitor<'de> for Visitor<'_, 'de, Inner, Extra>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions,
{
    type Value = Inner::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.as_ref().expect("the inner visitor has not been consumed while the external deserializer is running").expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_bool(v);

        framework(
            self,
            |visitor| visitor.visit_bool(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i8(v);

        framework(
            self,
            |visitor| visitor.visit_i8(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i16(v);

        framework(
            self,
            |visitor| visitor.visit_i16(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i32(v);

        framework(
            self,
            |visitor| visitor.visit_i32(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i64(v);

        framework(
            self,
            |visitor| visitor.visit_i64(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i128(v);

        framework(
            self,
            |visitor| visitor.visit_i128(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u8(v);

        framework(
            self,
            |visitor| visitor.visit_u8(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u16(v);

        framework(
            self,
            |visitor| visitor.visit_u16(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u32(v);

        framework(
            self,
            |visitor| visitor.visit_u32(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u64(v);

        framework(
            self,
            |visitor| visitor.visit_u64(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u128(v);

        framework(
            self,
            |visitor| visitor.visit_u128(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_f32(v);

        framework(
            self,
            |visitor| visitor.visit_f32(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_f64(v);

        framework(
            self,
            |visitor| visitor.visit_f64(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_char(v);

        framework(
            self,
            |visitor| visitor.visit_char(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_str(v);

        framework(
            self,
            |visitor| visitor.visit_str(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_borrowed_str(v);

        framework(
            self,
            |visitor| visitor.visit_borrowed_str(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_string(&v);

        framework(
            self,
            |visitor| visitor.visit_string(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_bytes(v);

        framework(
            self,
            |visitor| visitor.visit_bytes(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global
            .reporter
            .report_recv_visit_start_borrowed_bytes(v);

        framework(
            self,
            |visitor| visitor.visit_borrowed_bytes(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_byte_buf(&v);

        framework(
            self,
            |visitor| visitor.visit_byte_buf(v),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_none();

        framework(
            self,
            |visitor| visitor.visit_none(),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_unit();

        framework(
            self,
            |visitor| visitor.visit_unit(),
            |reporter, error| {
                reporter.report_recv_visit_end_primitive(error);
            },
        )
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
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
