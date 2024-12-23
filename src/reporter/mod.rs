use std::error::Error as StdError;

mod default_reporter;

pub use default_reporter::DefaultReporter;

static_assertions::assert_obj_safe!(Reporter);

pub trait Reporter {
    /// This is called after visiting anything that doesn't have its own
    /// `report_end_*` method.
    fn report_end_visit_primitive(&mut self, error: Option<&(dyn StdError + 'static)>);
    fn report_start_visit_bool(&mut self, v: bool);
    fn report_start_visit_i8(&mut self, v: i8);
    fn report_start_visit_i16(&mut self, v: i16);
    fn report_start_visit_i32(&mut self, v: i32);
    fn report_start_visit_i64(&mut self, v: i64);
    fn report_start_visit_i128(&mut self, v: i128);
    fn report_start_visit_u8(&mut self, v: u8);
    fn report_start_visit_u16(&mut self, v: u16);
    fn report_start_visit_u32(&mut self, v: u32);
    fn report_start_visit_u64(&mut self, v: u64);
    fn report_start_visit_u128(&mut self, v: u128);
    fn report_start_visit_f32(&mut self, v: f32);
    fn report_start_visit_f64(&mut self, v: f64);
    fn report_start_visit_char(&mut self, v: char);
    fn report_start_visit_str(&mut self, v: &str);
    fn report_start_visit_borrowed_bytes(&mut self, v: &[u8]);
    fn report_start_visit_byte_buf(&mut self, v: &Vec<u8>);
    fn report_start_visit_none(&mut self);
    fn report_start_visit_some(&mut self);
    fn report_end_visit_some(&mut self, error: Option<&(dyn StdError + 'static)>);
    fn report_start_visit_unit(&mut self);
    fn report_start_visit_newtype_struct(&mut self);
    fn report_end_visit_newtype_struct(&mut self, error: Option<&(dyn StdError + 'static)>);
    fn report_start_visit_seq(&mut self);
    fn report_end_visit_seq(&mut self, error: Option<&(dyn StdError + 'static)>);
    fn report_start_visit_map(&mut self);
    fn report_end_visit_map(&mut self, error: Option<&(dyn StdError + 'static)>);
    fn report_start_visit_enum(&mut self);
    fn report_end_visit_enum(&mut self, error: Option<&(dyn StdError + 'static)>);
}
