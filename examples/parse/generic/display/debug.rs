use crate::generic::format::ParseResult;

use super::ActiveDisplay;

pub struct Display {
    pub prefix: &'static str,
}

impl ActiveDisplay for Display {
    fn display(&mut self, value: &ParseResult) -> String {
        format!("{}{:?}", self.prefix, value)
    }
}
