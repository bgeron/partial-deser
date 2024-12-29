#[cfg(doc)]
use crate::fallback;
use std::error::Error as StdError;

mod default_reporter;

pub use default_reporter::DefaultReporter;

static_assertions::assert_obj_safe!(Reporter);

pub trait Reporter<'deserializer_error> {
    fn report_deserialize_start_any(&mut self);
    fn report_deserialize_start_bool(&mut self);
    fn report_deserialize_start_i8(&mut self);
    fn report_deserialize_start_i16(&mut self);
    fn report_deserialize_start_i32(&mut self);
    fn report_deserialize_start_i64(&mut self);
    fn report_deserialize_start_i128(&mut self);
    fn report_deserialize_start_u8(&mut self);
    fn report_deserialize_start_u16(&mut self);
    fn report_deserialize_start_u32(&mut self);
    fn report_deserialize_start_u64(&mut self);
    fn report_deserialize_start_u128(&mut self);
    fn report_deserialize_start_f32(&mut self);
    fn report_deserialize_start_f64(&mut self);
    fn report_deserialize_start_char(&mut self);
    fn report_deserialize_start_str(&mut self);
    fn report_deserialize_start_string(&mut self);
    fn report_deserialize_start_bytes(&mut self);
    fn report_deserialize_start_byte_buf(&mut self);
    fn report_deserialize_start_option(&mut self);
    fn report_deserialize_start_unit(&mut self);
    fn report_deserialize_start_unit_struct(&mut self, name: &'static str);
    fn report_deserialize_start_newtype_struct(&mut self, name: &'static str);
    fn report_deserialize_start_seq(&mut self);
    fn report_deserialize_start_tuple(&mut self, len: usize);
    fn report_deserialize_start_tuple_struct(&mut self, name: &'static str, len: usize);
    fn report_deserialize_start_map(&mut self);
    fn report_deserialize_start_struct(
        &mut self,
        name: &'static str,
        fields: &'static [&'static str],
    );
    fn report_deserialize_start_enum(
        &mut self,
        name: &'static str,
        variants: &'static [&'static str],
    );
    fn report_deserialize_start_identifier(&mut self);
    fn report_deserialize_start_ignored_any(&mut self);
    fn report_deserialize_end(&mut self, error: Option<&(dyn StdError + 'deserializer_error)>);

    /// This is called after visiting anything that doesn't have its own
    /// `report_end_*` method.
    fn report_recv_visit_end_primitive(&mut self, error: Option<&(dyn StdError + 'deserializer_error)>);
    fn report_recv_visit_start_bool(&mut self, v: bool);
    fn report_recv_visit_start_i8(&mut self, v: i8);
    fn report_recv_visit_start_i16(&mut self, v: i16);
    fn report_recv_visit_start_i32(&mut self, v: i32);
    fn report_recv_visit_start_i64(&mut self, v: i64);
    fn report_recv_visit_start_i128(&mut self, v: i128);
    fn report_recv_visit_start_u8(&mut self, v: u8);
    fn report_recv_visit_start_u16(&mut self, v: u16);
    fn report_recv_visit_start_u32(&mut self, v: u32);
    fn report_recv_visit_start_u64(&mut self, v: u64);
    fn report_recv_visit_start_u128(&mut self, v: u128);
    fn report_recv_visit_start_f32(&mut self, v: f32);
    fn report_recv_visit_start_f64(&mut self, v: f64);
    fn report_recv_visit_start_char(&mut self, v: char);
    fn report_recv_visit_start_str(&mut self, v: &str);
    fn report_recv_visit_start_borrowed_bytes(&mut self, v: &[u8]);
    fn report_recv_visit_start_byte_buf(&mut self, v: &Vec<u8>);
    fn report_recv_visit_start_none(&mut self);
    fn report_recv_visit_start_some(&mut self);
    fn report_recv_visit_end_some(&mut self, error: Option<&(dyn StdError + 'deserializer_error)>);
    fn report_recv_visit_start_unit(&mut self);
    fn report_recv_visit_start_newtype_struct(&mut self);
    fn report_recv_visit_end_newtype_struct(&mut self, error: Option<&(dyn StdError + 'deserializer_error)>);
    fn report_recv_visit_start_seq(&mut self);
    fn report_recv_visit_end_seq(&mut self, error: Option<&(dyn StdError + 'deserializer_error)>);
    fn report_recv_visit_start_map(&mut self);
    fn report_recv_visit_end_map(&mut self, error: Option<&(dyn StdError + 'deserializer_error)>);
    fn report_recv_visit_start_enum(&mut self);
    fn report_recv_visit_end_enum(&mut self, error: Option<&(dyn StdError + 'deserializer_error)>);

    /// The deserializer failed without consuming the visitor. We start computing a [`fallback`].
    fn report_start_fallback(&mut self);

    /// The deserializer failed without consuming the visitor, but we chose not to apply
    /// a [`fallback`]. (This is reported when the fallback does not take the visitor.)
    fn report_no_fallback(&mut self);

    /// The deserializer failed without consuming the visitor, and a [`fallback`] was applied,
    /// or at least attempted.
    fn report_fallback(&mut self, error: Option<&(dyn StdError + 'deserializer_error)>);
}
