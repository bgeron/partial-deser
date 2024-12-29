use std::error::Error as StdError;
#[cfg(feature = "tracing")]
use tracing::trace;

use super::{DeserializeStartArgs, DeserializeStartArgsExt, Reporter};

#[cfg(not(feature = "tracing"))]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

/// A reporter that logs on tracing (if that crate is enabled), or does
/// nothing otherwise.
#[derive(Debug, Default)]
pub struct DefaultReporter {
    /// nesting level
    level: usize,
}

impl DefaultReporter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Reporter for DefaultReporter {
    fn report_deserialize_start_any(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_any");
    }

    fn report_deserialize_start_bool(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_bool");
    }

    fn report_deserialize_start_i8(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_i8");
    }

    fn report_deserialize_start_i16(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_i16");
    }

    fn report_deserialize_start_i32(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_i32");
    }

    fn report_deserialize_start_i64(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_i64");
    }

    fn report_deserialize_start_i128(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_i128");
    }

    fn report_deserialize_start_u8(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_u8");
    }

    fn report_deserialize_start_u16(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_u16");
    }

    fn report_deserialize_start_u32(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_u32");
    }

    fn report_deserialize_start_u64(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_u64");
    }

    fn report_deserialize_start_u128(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_u128");
    }

    fn report_deserialize_start_f32(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_f32");
    }

    fn report_deserialize_start_f64(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_f64");
    }

    fn report_deserialize_start_char(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_char");
    }

    fn report_deserialize_start_str(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_str");
    }

    fn report_deserialize_start_string(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_string");
    }

    fn report_deserialize_start_bytes(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_bytes");
    }

    fn report_deserialize_start_byte_buf(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_byte_buf");
    }

    fn report_deserialize_start_option(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_option");
    }

    fn report_deserialize_start_unit(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_unit");
    }

    fn report_deserialize_start_unit_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
    ) {
        trace!(
            nesting_level = self.level,
            name,
            expecting = %args.expecting(), "start deserialize_unit_struct"
        );
    }

    fn report_deserialize_start_newtype_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
    ) {
        trace!(
            nesting_level = self.level,
            name,
            expecting = %args.expecting(), "start deserialize_newtype_struct"
        );
    }

    fn report_deserialize_start_seq(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_seq");
    }

    fn report_deserialize_start_tuple(&mut self, args: impl DeserializeStartArgs, len: usize) {
        trace!(nesting_level = self.level, len, expecting = %args.expecting(), "start deserialize_tuple");
    }

    fn report_deserialize_start_tuple_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
        len: usize,
    ) {
        trace!(
            nesting_level = self.level,
            name,
            len,
            expecting = %args.expecting(), "start deserialize_tuple_struct"
        );
    }

    fn report_deserialize_start_map(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_map");
    }

    fn report_deserialize_start_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
        fields: &'static [&'static str],
    ) {
        trace!(
            nesting_level = self.level,
            name,
            ?fields,
            expecting = %args.expecting(), "start deserialize_struct"
        );
    }

    fn report_deserialize_start_enum(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
        variants: &'static [&'static str],
    ) {
        trace!(
            nesting_level = self.level,
            name,
            ?variants,
            expecting = %args.expecting(), "start deserialize_enum"
        );
    }

    fn report_deserialize_start_identifier(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_identifier");
    }

    fn report_deserialize_start_ignored_any(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level, expecting = %args.expecting(), "start deserialize_ignored_any");
    }

    fn report_deserialize_end(&mut self, error: Option<&dyn StdError>) {
        // We would like to log errors as tracing::Value, but that requires the error
        // type to be 'static. Which we can make it for our deserializer (in this
        // method), but not for our visitor (in the `report_recv_visit_*` methods).
        trace!(
            nesting_level = self.level,
            error = error.map(tracing::field::display),
            "end deserialize"
        );
    }

    fn report_recv_visit_end_primitive(&mut self, error: Option<&dyn StdError>) {
        trace!(
            nesting_level = self.level,
            error = error.map(tracing::field::display),
            "end receive visit_primitive"
        );
    }

    fn report_recv_visit_start_bool(&mut self, v: bool) {
        trace!(nesting_level = self.level, v, "receive visit_bool");
    }

    fn report_recv_visit_start_i8(&mut self, v: i8) {
        trace!(nesting_level = self.level, v, "receive visit_i8");
    }

    fn report_recv_visit_start_i16(&mut self, v: i16) {
        trace!(nesting_level = self.level, v, "receive visit_i16");
    }

    fn report_recv_visit_start_i32(&mut self, v: i32) {
        trace!(nesting_level = self.level, v, "receive visit_i32");
    }

    fn report_recv_visit_start_i64(&mut self, v: i64) {
        trace!(nesting_level = self.level, v, "receive visit_i64");
    }

    fn report_recv_visit_start_i128(&mut self, v: i128) {
        trace!(nesting_level = self.level, v, "receive visit_i128");
    }

    fn report_recv_visit_start_u8(&mut self, v: u8) {
        trace!(nesting_level = self.level, v, "receive visit_u8");
    }

    fn report_recv_visit_start_u16(&mut self, v: u16) {
        trace!(nesting_level = self.level, v, "receive visit_u16");
    }

    fn report_recv_visit_start_u32(&mut self, v: u32) {
        trace!(nesting_level = self.level, v, "receive visit_u32");
    }

    fn report_recv_visit_start_u64(&mut self, v: u64) {
        trace!(nesting_level = self.level, v, "receive visit_u64");
    }

    fn report_recv_visit_start_u128(&mut self, v: u128) {
        trace!(nesting_level = self.level, v, "receive visit_u128");
    }

    fn report_recv_visit_start_f32(&mut self, v: f32) {
        trace!(nesting_level = self.level, v, "receive visit_f32");
    }

    fn report_recv_visit_start_f64(&mut self, v: f64) {
        trace!(nesting_level = self.level, v, "receive visit_f64");
    }

    fn report_recv_visit_start_char(&mut self, v: char) {
        trace!(nesting_level = self.level, ?v, "receive visit_char");
    }

    fn report_recv_visit_start_str(&mut self, v: &str) {
        trace!(nesting_level = self.level, v, "receive visit_str");
    }

    fn report_recv_visit_start_borrowed_bytes(&mut self, v: &[u8]) {
        trace!(
            nesting_level = self.level,
            v,
            "receive visit_borrowed_bytes"
        );
    }

    fn report_recv_visit_start_byte_buf(&mut self, v: &[u8]) {
        trace!(nesting_level = self.level, ?v, "receive visit_byte_buf");
    }

    fn report_recv_visit_start_none(&mut self) {
        trace!(nesting_level = self.level, "receive visit_none");
    }

    fn report_recv_visit_start_some(&mut self) {
        trace!(nesting_level = self.level, "receive visit_some");
        self.level += 1;
    }

    fn report_recv_visit_end_some(&mut self, error: Option<&dyn StdError>) {
        self.level -= 1;
        trace!(
            nesting_level = self.level,
            error = error.map(tracing::field::display),
            "end receive visit_some"
        );
    }

    fn report_recv_visit_start_unit(&mut self) {
        trace!(nesting_level = self.level, "receive visit_unit");
    }

    fn report_recv_visit_start_newtype_struct(&mut self) {
        trace!(nesting_level = self.level, "receive visit_newtype_struct");
        self.level += 1;
    }

    fn report_recv_visit_end_newtype_struct(&mut self, error: Option<&dyn StdError>) {
        self.level -= 1;
        trace!(
            nesting_level = self.level,
            error = error.map(tracing::field::display),
            "end receive visit_newtype_struct"
        );
    }

    fn report_recv_visit_start_seq(&mut self) {
        trace!(nesting_level = self.level, "receive visit_seq");
        self.level += 1;
    }

    fn report_recv_visit_end_seq(&mut self, error: Option<&dyn StdError>) {
        self.level -= 1;
        trace!(
            nesting_level = self.level,
            error = error.map(tracing::field::display),
            "end receive visit_seq"
        );
    }

    fn report_recv_visit_start_map(&mut self) {
        trace!(nesting_level = self.level, "receive visit_map");
        self.level += 1;
    }

    fn report_recv_visit_end_map(&mut self, error: Option<&dyn StdError>) {
        self.level -= 1;
        trace!(
            nesting_level = self.level,
            error = error.map(tracing::field::display),
            "end receive visit_map"
        );
    }

    fn report_recv_visit_start_enum(&mut self) {
        trace!(nesting_level = self.level, "receive visit_enum");
        self.level += 1;
    }

    fn report_recv_visit_end_enum(&mut self, error: Option<&dyn StdError>) {
        self.level -= 1;
        trace!(
            nesting_level = self.level,
            error = error.map(tracing::field::display),
            "end receive visit_enum"
        );
    }

    fn report_start_fallback(&mut self) {}

    fn report_no_fallback(&mut self) {
        trace!(nesting_level = self.level, "no fallback was attempted");
    }

    fn report_fallback(&mut self, error: Option<&dyn StdError>) {
        if let Some(error) = error {
            trace!(
                nesting_level = self.level,
                error = %error,
                "fallback attempted but failed"
            );
        } else {
            trace!(nesting_level = self.level, "fallback applied");
        }
    }
}
