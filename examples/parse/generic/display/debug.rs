use futures::future::BoxFuture;
use futures::FutureExt;

use crate::generic::format::ParseResult;

use super::ActiveDisplay;

pub struct Display {
    pub prefix: &'static str,
}

impl ActiveDisplay for Display {
    fn display(&mut self, value: ParseResult) -> BoxFuture<String> {
        let displayed = format!("{}{:?}", self.prefix, value);
        async { displayed }.boxed()
    }
}
