//! Deserialize with Serde from partial JSON and more
//!
//! This crate reads incomplete JSON and parses it for your
//! data structures that implement [`Deserialize`]:
//!
//! ```
//! # use serde::Deserialize;
//! #[derive(Debug, Deserialize)]
//! struct TravelMode {
//!    mode: String,
//!    benefit: Option<String>
//! }
//!
//! let json = r#"[{"mode": "foot", "benefit": "healthy"}, {"mode": "aeropl"#;
//! let modes: Vec<TravelMode> = serde_partial::from_json_str(json).unwrap();
//! assert_eq!(format!("{modes:?}"), r#"[
//!    TravelMode { mode: "foot", benefit: Some("healthy") },
//!    TravelMode { mode: "aeropl", benefit: None }
//! ]"#);
//! ```
//!
//! This crate is generic for many or all data formats, not just JSON. There is merely
//! a tweak specific to JSON to be able to parse unfinished strings.
//!
//! <!-- todo: list other data formats that work -->
//!
//! ## How this works
//!
//! todo
//!
//! ## Limitations
//!
//! Partial deserialization
//!
//! -
//!

use std::sync::Arc;

mod error;
mod fallback;
mod r#impl;
mod options;
mod reporter;
mod source;

pub use error::Error;
pub use options::DefaultExtraOptions;
use options::ExtraOptions;
pub use source::Source;
#[cfg(feature = "unstable")]
pub use {fallback::Fallbacks, reporter::Reporter};

#[cfg(feature = "serde_json")]
const RANDOM_PARTIAL_JSON_TAG_LEN: usize = 8;

#[derive(Clone, Debug)]
pub struct Options<Extra: ExtraOptions = DefaultExtraOptions> {
    /// This is a random string that forms part of a suffix we add to
    /// the input JSON.
    ///
    /// As of Dec 2024, we don't stabilize the specific string format.
    #[cfg(feature = "serde_json")]
    parse_partial_json_tag: Option<Arc<str>>,

    extra: Extra,
}

/// Partially deserialize the input with [`serde_json`].
#[cfg(feature = "serde_json")]
pub fn from_json_str<T>(json: &str) -> Result<T, Error<serde_json::Error>> {
    Options::new_json().from_json_str(json)
}

impl Options {
    /// Default config for JSON.
    ///
    /// This currently will generate a short random string for improved deserialization of
    /// partial strings.
    #[cfg(feature = "serde_json")]
    pub fn new_json() -> Options<DefaultExtraOptions> {
        use rand::distributions::{Alphanumeric, DistString};
        use rand::thread_rng;

        let tag = Alphanumeric.sample_string(&mut thread_rng(), RANDOM_PARTIAL_JSON_TAG_LEN);
        Self {
            parse_partial_json_tag: Some(tag.into()),
            ..Self::new_generic()
        }
    }

    pub fn new_generic() -> Options<DefaultExtraOptions> {
        Self {
            #[cfg(feature = "serde_json")]
            parse_partial_json_tag: None,
            extra: DefaultExtraOptions,
        }
    }

    #[cfg(feature = "serde_json")]
    pub fn from_json_str<T>(&self, json: &str) -> Result<T, Error<serde_json::Error>> {
        todo!()
    }
}
