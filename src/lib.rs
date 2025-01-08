#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(
    not(feature = "tracing"),
    allow(unused_variables, unused_imports, dead_code)
)]

//! Deserialize with Serde from partial JSON and more
//!
//! This crate reads incomplete JSON and parses it for your
//! data structures that implement [`Deserialize`]:
//!
//! ```
//! # use serde::Deserialize;
//! #[derive(Debug, Deserialize, PartialEq)]
//! struct TravelMode {
//!    mode: String,
//!    benefit: Option<String>
//! }
//!
//! let json = r#"[{"mode": "foot", "benefit": "healthy"}, {"mode": "aeropl"#;
//! let modes: Vec<TravelMode> = partial_deser::from_json_str(json).unwrap();
//! assert_eq!(modes, [
//!    TravelMode { mode: "foot".to_string(), benefit: Some("healthy".to_string()) },
//!    TravelMode { mode: "aeropl".to_string(), benefit: None }
//! ]);
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

#[cfg(feature = "serde_json")]
use std::sync::Arc;

#[cfg(doc)]
use serde::Deserialize;

macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        ::tracing::error!($($arg)*)
    };
}
macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        ::tracing::warn!($($arg)*)
    };
}
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        ::tracing::debug!($($arg)*)
    };
}
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        ::tracing::trace!($($arg)*)
    };
}

mod attempt;
mod deserialize;
mod error;
mod fallback;
mod options;
mod reporter;
pub mod source;
mod state;
mod util;

/// Reexports to satisfy Rust's visibility rules TODO
#[allow(unused_imports)]
pub mod unstable {
    pub use crate::fallback::Fallbacks;
    pub use crate::options::{
        ExtraOptionsStruct, MakeFallbackProvider, MakeReporter, UnstableCustomBehavior,
    };
    pub use crate::reporter::Reporter;
}

pub use error::Error;
pub use options::{DefaultExtraOptions, MakeDefaultFallbacks, MakeDefaultReporter};
use options::{ExtraOptions, UnstableCustomBehavior};
pub use source::Source;

#[cfg(feature = "serde_json")]
const RANDOM_PARTIAL_JSON_TAG_LEN: usize = 8;

/// Number of times that we may backtrack.
///
/// For good results, you should allow at least one backtracking for when the input
/// stops in the middle of a map/struct value or enum.
///
/// A higher limit on backtracks is useful when not all struct fields
/// are declared `#[serde(default)]`. In this case, the algorithm will attempt to
/// incrementally prune on higher levels, e.g. omitting the list item that contains
/// the end-of-file, or omitting a field of an enclosing struct.
const DEFAULT_MAX_BACKTRACKS: Option<usize> = Some(10);

#[derive(Clone, Debug)]
pub struct Options<Extra: ExtraOptions = DefaultExtraOptions> {
    /// This is a random string that forms part of a suffix we add to
    /// the input JSON.
    ///
    /// As of Dec 2024, we don't stabilize the specific string format.
    #[cfg(feature = "serde_json")]
    parse_partial_json_tag: Option<Arc<str>>,

    max_n_backtracks: Option<usize>,

    behavior: UnstableCustomBehavior,

    extra: Extra,
}

/// Partially deserialize the input with [`serde_json`].
#[cfg(feature = "serde_json")]
pub fn from_json_str<'de, T>(json: &'de str) -> Result<T, Error<serde_json::Error>>
where
    T: serde::Deserialize<'de>,
{
    Options::new_json().from_json_str(json)
}

/// Partially deserialize the input with [`serde_json`].
#[cfg(feature = "serde_json")]
pub fn from_json_slice<'de, T>(json: &'de [u8]) -> Result<T, Error<serde_json::Error>>
where
    T: serde::Deserialize<'de>,
{
    Options::new_json().from_json_slice(json)
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
        Options {
            parse_partial_json_tag: Some(tag.into()),
            ..Options::new_generic()
        }
    }

    pub fn new_generic() -> Options<DefaultExtraOptions> {
        Options {
            #[cfg(feature = "serde_json")]
            parse_partial_json_tag: None,
            max_n_backtracks: DEFAULT_MAX_BACKTRACKS,
            behavior: UnstableCustomBehavior::default(),
            extra: DefaultExtraOptions::default(),
        }
    }

    pub fn with_max_n_backtracks(mut self, max_n_backtracks: Option<usize>) -> Self {
        self.max_n_backtracks = max_n_backtracks;
        self
    }

    #[cfg(feature = "serde_json")]
    pub fn from_json_str<'de, T>(self, json: &'de str) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(source::JsonStr(json))
    }

    #[cfg(feature = "serde_json")]
    pub fn from_json_slice<'de, T>(self, json: &'de [u8]) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(source::JsonBytes(json))
    }

    #[cfg(feature = "unstable")]
    pub fn custom_behavior(self, behavior: UnstableCustomBehavior) -> Self {
        Options { behavior, ..self }
    }
}
