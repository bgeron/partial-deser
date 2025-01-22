use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use super::format::{FormatAndSettings, ParseResult};

/// The schema that the output should conform to.
///
/// This can significantly change the result, both larger (because `#[serde(default)]`
/// adds fields) and smaller (when a partial list element isn't parseable at all).
#[derive(Debug, Clone, ValueEnum, Default)]
pub enum Schema {
    /// Any JSON value.
    #[default]
    Any,
    /// A list of structs with mode and benefit fields, both strings.
    TravelModes,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(default)]
struct TravelMode {
    mode: String,
    benefit: Option<String>,
}

impl Schema {
    pub fn parse(&self, format: &FormatAndSettings, input: &[u8]) -> ParseResult {
        match self {
            Schema::Any => format.parse::<serde_json::Value>(input),
            Schema::TravelModes => format.parse::<Vec<TravelMode>>(input),
        }
    }
}
