use clap::ValueEnum;
use futures::future::BoxFuture;

use super::format::ParseResult;

mod debug;
mod nu;

/// How to display the parsed value.
///
/// By default, this will use nushell, falling back to Debug.
#[derive(Debug, Clone, ValueEnum)]

pub enum Display {
    Nushell,
    Debug,
}

impl Display {
    /// Display types need to be initialized to do some startup.
    pub async fn init(method: &Option<Self>) -> Box<dyn ActiveDisplay> {
        match method {
            None => {
                // Autodetect nu. Otherwise fall back to debug.
                match nu::Display::new_if_nu_installed().await {
                    Some(nu) => Box::new(nu),
                    None => Box::new(debug::Display {
                        prefix: "Nushell does not seem to be present, falling back to Debug.\n\n",
                    }),
                }
            }
            Some(Display::Nushell) => Box::new(nu::Display::new_always()),
            Some(Display::Debug) => Box::new(debug::Display { prefix: "" }),
        }
    }
}

pub trait ActiveDisplay: Send {
    fn display(&mut self, value: ParseResult) -> BoxFuture<String>;
}
