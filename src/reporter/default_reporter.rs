use std::error::Error as StdError;
#[cfg(feature = "tracing")]
use tracing::trace;

use super::Reporter;

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
    fn report_end_visit_primitive(&mut self, error: Option<&(dyn StdError + 'static)>) {
        trace!(
            nesting_level = self.level,
            error = error,
            "end_visit_primitive"
        );
    }

    fn report_start_visit_bool(&mut self, v: bool) {
        trace!(nesting_level = self.level, v, "visit_bool");
    }

    fn report_start_visit_i8(&mut self, v: i8) {
        trace!(nesting_level = self.level, v, "visit_i8");
    }

    fn report_start_visit_i16(&mut self, v: i16) {
        trace!(nesting_level = self.level, v, "visit_i16");
    }

    fn report_start_visit_i32(&mut self, v: i32) {
        trace!(nesting_level = self.level, v, "visit_i32");
    }

    fn report_start_visit_i64(&mut self, v: i64) {
        trace!(nesting_level = self.level, v, "visit_i64");
    }

    fn report_start_visit_i128(&mut self, v: i128) {
        trace!(nesting_level = self.level, v, "visit_i128");
    }

    fn report_start_visit_u8(&mut self, v: u8) {
        trace!(nesting_level = self.level, v, "visit_u8");
    }

    fn report_start_visit_u16(&mut self, v: u16) {
        trace!(nesting_level = self.level, v, "visit_u16");
    }

    fn report_start_visit_u32(&mut self, v: u32) {
        trace!(nesting_level = self.level, v, "visit_u32");
    }

    fn report_start_visit_u64(&mut self, v: u64) {
        trace!(nesting_level = self.level, v, "visit_u64");
    }

    fn report_start_visit_u128(&mut self, v: u128) {
        trace!(nesting_level = self.level, v, "visit_u128");
    }

    fn report_start_visit_f32(&mut self, v: f32) {
        trace!(nesting_level = self.level, v, "visit_f32");
    }

    fn report_start_visit_f64(&mut self, v: f64) {
        trace!(nesting_level = self.level, v, "visit_f64");
    }

    fn report_start_visit_char(&mut self, v: char) {
        trace!(nesting_level = self.level, ?v, "visit_char");
    }

    fn report_start_visit_str(&mut self, v: &str) {
        trace!(nesting_level = self.level, v, "visit_str");
    }

    fn report_start_visit_borrowed_bytes(&mut self, v: &[u8]) {
        trace!(nesting_level = self.level, v, "visit_borrowed_bytes");
    }

    fn report_start_visit_byte_buf(&mut self, v: &Vec<u8>) {
        trace!(nesting_level = self.level, ?v, "visit_byte_buf");
    }

    fn report_start_visit_none(&mut self) {
        trace!(nesting_level = self.level, "visit_none");
    }

    fn report_start_visit_some(&mut self) {
        trace!(nesting_level = self.level, "visit_some");
        self.level += 1;
    }

    fn report_end_visit_some(&mut self, error: Option<&(dyn StdError + 'static)>) {
        self.level -= 1;
        trace!(nesting_level = self.level, error = error, "end_visit_some");
    }

    fn report_start_visit_unit(&mut self) {
        trace!(nesting_level = self.level, "visit_unit");
    }

    fn report_start_visit_newtype_struct(&mut self) {
        trace!(nesting_level = self.level, "visit_newtype_struct");
        self.level += 1;
    }

    fn report_end_visit_newtype_struct(&mut self, error: Option<&(dyn StdError + 'static)>) {
        self.level -= 1;
        trace!(
            nesting_level = self.level,
            error = error,
            "end_visit_newtype_struct"
        );
    }

    fn report_start_visit_seq(&mut self) {
        trace!(nesting_level = self.level, "visit_seq");
        self.level += 1;
    }

    fn report_end_visit_seq(&mut self, error: Option<&(dyn StdError + 'static)>) {
        self.level -= 1;
        trace!(nesting_level = self.level, error = error, "end_visit_seq");
    }

    fn report_start_visit_map(&mut self) {
        trace!(nesting_level = self.level, "visit_map");
        self.level += 1;
    }

    fn report_end_visit_map(&mut self, error: Option<&(dyn StdError + 'static)>) {
        self.level -= 1;
        trace!(nesting_level = self.level, error = error, "end_visit_map");
    }

    fn report_start_visit_enum(&mut self) {
        trace!(nesting_level = self.level, "visit_enum");
        self.level += 1;
    }

    fn report_end_visit_enum(&mut self, error: Option<&(dyn StdError + 'static)>) {
        self.level -= 1;
        trace!(nesting_level = self.level, error = error, "end_visit_enum");
    }
}
