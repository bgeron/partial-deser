use std::default;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use super::format::{FormatAndSettings, ParseResult, Parsed};

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
    pub fn init(method: &Option<Self>) -> Box<dyn ActiveDisplay> {
        match method {
            None => {
                // Autodetect nu. Otherwise fall back to debug.
                match nu::Display::new_if_nu_installed() {
                    Some(nu) => Box::new(nu),
                    None => Box::new(debug::Display {
                        prefix: "Nushell does not seem to be present, falling back to Debug.\n\n",
                    }),
                }
            }
            _ => todo!(),
            // Some(Display::Nushell) => Box::new(nu::Display),
            // Some(Display::Debug) => Box::new(debug::Display),
        }
    }
}

pub trait ActiveDisplay {
    fn display(&mut self, value: &ParseResult) -> String;
}
