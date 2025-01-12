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
//! ## Caveats
//!
//! The JSON trick can sometimes TODO (but we disabled fallbacks as appropriate?)
//!
//! ## Criteria
//!
//! - I tried that the empty string always parses as something
//!    
//!   .. but for enums this is not possible
//!
//! - I tried that with more input, it never takes something away

#[cfg(feature = "serde_json")]
use std::borrow::Cow;
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
#[cfg(feature = "serde_json")]
mod json_trick;
mod options;
mod reporter;
pub mod source;
mod state;
mod string_like;
mod util;

/// Reexports to satisfy Rust's visibility rules TODO
#[cfg(feature = "unstable")]
#[allow(unused_imports)]
pub mod unstable {
    pub use crate::fallback::Fallbacks;
    #[cfg(feature = "serde_json")]
    pub mod json_trick {
        pub use crate::json_trick::Prepared;
    }
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
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(feature = "serde_json")]
pub fn from_json_str<T>(json: Cow<str>) -> Result<T, Error<serde_json::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    Options::new_json().from_json_str(json)
}

/// Like [`from_json_str`], but for bytes.
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(feature = "serde_json")]
pub fn from_json_slice<T>(json: Cow<[u8]>) -> Result<T, Error<serde_json::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
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

    /// Like [`crate::from_json_str`], but with options.
    #[cfg(feature = "serde_json")]
    pub fn from_json_str<'a, T>(self, json: Cow<'a, str>) -> Result<T, Error<serde_json::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_str_for_borrowed_deserialization(json);
        self.from_json_str_borrowed(&prepared)
    }

    /// Like [`crate::from_json_slice`], but with options.
    #[cfg(feature = "serde_json")]
    pub fn from_json_slice<T>(self, json: Cow<[u8]>) -> Result<T, Error<serde_json::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_slice_for_borrowed_deserialization(json);
        self.from_json_slice_borrowed(&prepared)
    }

    /// Like [`Self::from_json_slice`], but can deserialize borrowed strings and return them
    /// directly.
    ///
    /// This comes at the cost that we cannot use the JSON trick that gets us the contents of
    /// incomplete strings.
    ///
    /// If you need incomplete strings as well, then use [`Self::from_json_slice_borrowed`].
    ///
    /// ```
    /// # use serde::Deserialize;
    /// #[derive(Debug, Deserialize, PartialEq)]
    /// struct TravelMode {
    ///    mode: String,
    ///    benefit: Option<String>
    /// }
    ///
    /// let json = r#"[{"mode": "foot", "benefit": "healthy"}, {"mode": "aeropl"#;
    /// let modes: Vec<TravelMode> = partial_deser::from_json_str(json).unwrap();
    /// assert_eq!(modes, [
    ///    TravelMode { mode: "foot".to_string(), benefit: Some("healthy".to_string()) },
    ///    // Note: missing aeroplane
    /// ]);
    /// ```
    #[cfg(feature = "serde_json")]
    pub fn from_json_slice_plain_return_borrowed<'de, T>(
        self,
        json: &'de impl AsRef<[u8]>,
    ) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(source::JsonBytes(json.as_ref()))
    }

    /// Advanced API. Lets you deserialize into borrowed types like `&str`, while supporting
    /// the JSON trick that gets us the contents of incomplete strings.
    ///
    /// (The difference is that this only needs `T: serde::de::Deserialize<'de>`, which is weaker.)
    ///
    /// ```
    /// # use serde::Deserialize;
    /// /// Note: `&'a str` instead of `String`.
    /// ///
    /// /// Like with serde_json, deserializing to &str can fail. Instead, you should probably
    /// /// use `Cow<str>`, or just `String`.
    /// #[derive(Debug, Deserialize, PartialEq)]
    /// struct TravelMode<'a> {
    ///    mode: &'a str,
    ///    benefit: Option<&'a str>
    /// }
    ///
    /// let json = r#"[{"mode": "foot", "benefit": "healthy"}, {"mode": "aeropl"#;
    /// let modes: Vec<TravelMode> = partial_deser::from_json_str(json).unwrap();
    /// assert_eq!(modes, [
    ///    TravelMode { mode: "foot", benefit: Some("healthy") },
    ///    TravelMode { mode: "aeropl", benefit: None }
    /// ]);
    /// ```
    ///
    /// This is marked unstable because I'm not 100% sure about the [`unstable::json_trick::Prepared`]
    /// type. Input is welcome.
    #[cfg(all(feature = "serde_json", feature = "unstable"))]
    pub fn from_json_str_borrowed<'de, T>(
        self,
        json_trick::Prepared(prepared_json): &'de json_trick::Prepared<impl AsRef<str>>,
    ) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(source::JsonStr(prepared_json.as_ref()))
    }

    /// See [`Self::from_json_str_borrowed`].
    #[cfg(all(feature = "serde_json", feature = "unstable"))]
    pub fn from_json_slice_borrowed<'de, T>(
        self,
        json_trick::Prepared(prepared_json): &'de json_trick::Prepared<impl AsRef<[u8]>>,
    ) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(source::JsonBytes(prepared_json.as_ref()))
    }

    /// Prepare a string for borrowed deserialization with [`Self::from_json_str_borrowed`].
    ///
    /// This only appends to the input. And this returns a newtype wrapper, so you can undo
    /// the effects yourself.
    #[cfg(all(feature = "serde_json", feature = "unstable"))]
    pub fn prepare_str_for_borrowed_deserialization<'a>(
        &self,
        mut input: Cow<'a, str>,
    ) -> json_trick::Prepared<Cow<'a, str>> {
        if let Some(tag) = self.parse_partial_json_tag.as_ref() {
            json_trick::prepare_string_with_tag(tag, Cow::to_mut(&mut input));
        }
        json_trick::Prepared(input)
    }

    /// Prepare a slice for borrowed deserialization with [`Self::from_json_slice_borrowed`].
    ///
    /// This only appends to the input. And this returns a newtype wrapper, so you can undo
    /// the effects yourself.
    #[cfg(all(feature = "serde_json", feature = "unstable"))]
    pub fn prepare_slice_for_borrowed_deserialization<'a>(
        &self,
        mut input: Cow<'a, [u8]>,
    ) -> json_trick::Prepared<Cow<'a, [u8]>> {
        if let Some(tag) = self.parse_partial_json_tag.as_ref() {
            json_trick::prepare_vec_with_tag(tag, Cow::to_mut(&mut input));
        }
        json_trick::Prepared(input)
    }

    #[cfg(feature = "unstable")]
    pub fn custom_behavior(self, behavior: UnstableCustomBehavior) -> Self {
        Options { behavior, ..self }
    }
}
