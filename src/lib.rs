#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(
    not(all(feature = "rand", feature = "tracing")),
    allow(unused_variables, unused_imports, dead_code, unused_mut)
)]

//! # Deserialize incomplete data with Serde
//!
//! This wraps Serde [`Deserializer`]s (like serde_json and serde_yaml) so you
//! can parse incomplete data and get an incomplete result.
//!
//! (todo video here)
//!
//! This crate makes parsing more robust by absorbing errors from the data format,
//! and then bringing the parse to a safe halt.
//!
//! todo graphic
//!
//! todo techniques section?
//!
//! todo in practice this seems desirable?
//!
//! TODO This crate reads incomplete JSON and parses it for your
//! data structures that implement [`Deserialize`]:
//!
//! ```
//! # use serde::Deserialize;
//! #[derive(Debug, Deserialize, PartialEq)]
//! struct TravelMode {
//!   #[serde(default)]
//!   mode: String,
//!   benefit: Option<String>
//! }
//!
//! let json = r#"[{"mode": "foot", "benefit": "healthy"}, {"mode": "aeropl"#;
//! let modes: Vec<TravelMode> = deser_incomplete::from_json_str(json).unwrap();
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
//! ## Goal
//!
//! - not reverting
//!
//! ## How this works
//!
//! - todo randomized trailer
//!
//! ## Tested support for data formats
//!
//! - JSON: works very well. This is what the library was tweaked for.
//! - YAML: ...
//!
//! ## Limitations
//!
//! - data format should be relatively greedy/online/whatever
//!
//! - incomplete strings tend to require a randomized trailer
//!
//! - cannot distinguish eof from invalid input
//!
//! - This approach lets us safely abort parsing and get a value, but
//!   we cannot skip over invalid segments of input. (For that you need
//!   an approach like tree-sitter.)
//!
//! ## Notes
//!
//! - JSON: Number cannot end with floating point period
//!
//! - YAML works very well, but is a bit less exhaustively tested than JSON.
//!   The randomized trailer is really important here.
//!
//! ## Criteria
//!
//! - I tried that the empty string always parses as something
//!
//!   .. but for enums this is not possible
//!
//! - I tried that with more input, it never takes something away

use std::borrow::Cow;
#[cfg(feature = "rand")]
use std::sync::Arc;

use options::DefaultExtraOptions;
#[cfg(doc)]
use serde::{Deserialize, Deserializer};

macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        ::tracing::error!($($arg)*)
    };
}
// macro_rules! warn {
//     ($($arg:tt)*) => {
//         #[cfg(feature = "tracing")]
//         ::tracing::warn!($($arg)*)
//     };
// }
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
mod options_impl;
#[cfg(feature = "rand")]
pub mod random_trailer;
#[cfg(not(feature = "rand"))]
mod random_trailer;
mod reporter;
pub mod source;
mod state;
mod util;

/// Relatively stable parts to specify options.
pub mod options {
    #[cfg(all(feature = "rand", feature = "serde_json"))]
    pub use crate::options_impl::JsonExtraOptions;
    #[cfg(all(feature = "rand", feature = "serde_yaml"))]
    pub use crate::options_impl::YamlExtraOptions;
    pub use crate::options_impl::{DefaultExtraOptions, MakeDefaultFallbacks, MakeDefaultReporter};
}

/// Reexports to satisfy Rust's visibility rules. These are not stable across
/// versions.
#[cfg(feature = "unstable")]
#[allow(unused_imports)]
pub mod unstable {
    pub use crate::fallback::Fallbacks;
    pub use crate::options_impl::{
        ExtraOptions, ExtraOptionsStruct, MakeFallbackProvider, MakeReporter,
        UnstableCustomBehavior,
    };
    pub use crate::reporter::Reporter;
}

pub use error::Error;
use options_impl::{ExtraOptions, ExtraOptionsStruct, UnstableCustomBehavior};
use random_trailer::InputPlusTrailer;
pub use source::Source;

#[cfg(feature = "rand")]
const RANDOM_TAG_LEN: usize = 8;

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
    /// the input, for some data types.
    ///
    /// As of Dec 2024, we don't stabilize the specific string format.
    #[cfg(feature = "rand")]
    random_tag: Option<Arc<str>>,

    max_n_backtracks: Option<usize>,

    behavior: UnstableCustomBehavior,

    extra: Extra,
}

/// Main function. Robustly deserialize incomplete input with [`serde_json`].
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(all(feature = "rand", feature = "serde_json"))]
pub fn from_json_str<T>(json: &str) -> Result<T, Error<serde_json::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    Options::new_json().from_json_str(Cow::Borrowed(json))
}

/// Like [`from_json_str`], but for bytes.
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(all(feature = "rand", feature = "serde_json"))]
pub fn from_json_slice<T>(json: &[u8]) -> Result<T, Error<serde_json::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    Options::new_json().from_json_slice(Cow::Borrowed(json))
}

/// Robustly deserialize incomplete input with [`serde_yaml`].
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(all(feature = "rand", feature = "serde_yaml"))]
pub fn from_yaml_str<T>(yaml: &str) -> Result<T, Error<serde_yaml::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    Options::new_yaml().from_yaml_str(Cow::Borrowed(yaml))
}

/// Like [`from_yaml_str`], but for bytes.
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(all(feature = "rand", feature = "serde_yaml"))]
pub fn from_yaml_slice<T>(yaml: &[u8]) -> Result<T, Error<serde_yaml::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    Options::new_yaml().from_yaml_slice(Cow::Borrowed(yaml))
}

impl Options {
    /// Default config for JSON.
    ///
    /// This will currently generate a short extra trailer on inputs
    /// for improved deserialization of incomplete JSON.
    #[cfg(all(feature = "rand", feature = "serde_json"))]
    pub fn new_json() -> Options<options_impl::JsonExtraOptions> {
        let base = Options {
            ..Options::new_nonce()
        };
        base.set_random_trailer(random_trailer::json::JsonRandomTrailer)
    }

    /// Default config for YAML.
    ///
    /// This will currently generate a short extra trailer on inputs
    /// for improved deserialization of incomplete YAML.
    ///
    /// For YAML in particular, this suffix is important to get
    /// good behavior.
    #[cfg(all(feature = "rand", feature = "serde_yaml"))]
    pub fn new_yaml() -> Options<options_impl::YamlExtraOptions> {
        let base = Options {
            ..Options::new_nonce()
        };
        base.set_random_trailer(random_trailer::yaml::YamlRandomTrailer)
    }

    /// Basic config, suitable for any data format.
    ///
    /// These options support adding a randomized trailer to the input.
    /// However, you should probably call [`Options::set_random_trailer`]
    /// to specify how this trailer should be removed from parsed strings.
    #[cfg(feature = "rand")]
    pub fn new_nonce() -> Options<DefaultExtraOptions> {
        use rand::distributions::{Alphanumeric, DistString};
        use rand::thread_rng;

        // In the future, this may change to only generate a single random
        // tag for the lifetime of the application.
        let tag = Alphanumeric.sample_string(&mut thread_rng(), RANDOM_TAG_LEN);
        Options {
            random_tag: Some(tag.into()),
            ..Options::new_no_nonce()
        }
    }

    /// Basic config, suitable for any data format. However, this
    /// config does not allow adding a randomized trailer to the input,
    /// which tends to benefit many formats.
    ///
    /// - For `serde_json`, this means you won't get incomplete strings deserialized
    ///
    /// - For `serde_yaml`, this means that your output will flicker, as it seems to
    ///   buffer lines somehow, and if a line has an unterminated string, then the
    ///   whole line will be missing.
    pub fn new_no_nonce() -> Options<DefaultExtraOptions> {
        Options {
            #[cfg(feature = "rand")]
            random_tag: None,
            max_n_backtracks: DEFAULT_MAX_BACKTRACKS,
            behavior: UnstableCustomBehavior::default(),
            extra: DefaultExtraOptions::default(),
        }
    }
}

impl<Extra: ExtraOptions> Options<Extra> {
    pub fn with_max_n_backtracks(mut self, max_n_backtracks: Option<usize>) -> Self {
        self.max_n_backtracks = max_n_backtracks;
        self
    }

    /// Like [`crate::from_json_str`], but with options.
    #[cfg(all(feature = "rand", feature = "serde_json"))]
    pub fn from_json_str<T>(self, json: Cow<str>) -> Result<T, Error<serde_json::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_str_for_borrowed_deserialization(json);
        self.from_json_str_borrowed(&prepared)
    }

    /// Like [`crate::from_json_slice`], but with options.
    #[cfg(all(feature = "rand", feature = "serde_json"))]
    pub fn from_json_slice<T>(self, json: Cow<[u8]>) -> Result<T, Error<serde_json::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_slice_for_borrowed_deserialization(json);
        self.from_json_slice_borrowed(&prepared)
    }

    /// Like [`crate::from_yaml_str`], but with options.
    #[cfg(all(feature = "rand", feature = "serde_yaml"))]
    pub fn from_yaml_str<T>(self, yaml: Cow<str>) -> Result<T, Error<serde_yaml::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_str_for_borrowed_deserialization(yaml);
        self.from_yaml_str_borrowed(&prepared)
    }

    /// Like [`crate::from_yaml_slice`], but with options.
    #[cfg(all(feature = "rand", feature = "serde_yaml"))]
    pub fn from_yaml_slice<T>(self, yaml: Cow<[u8]>) -> Result<T, Error<serde_yaml::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_slice_for_borrowed_deserialization(yaml);
        self.from_yaml_slice_borrowed(&prepared)
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
    ///    #[serde(default)]
    ///    mode: String,
    ///    benefit: Option<String>
    /// }
    ///
    /// let json = r#"[{"mode": "foot", "benefit": "healthy"}, {"mode": "incomplete"#;
    /// let modes: Vec<TravelMode> = deser_incomplete::Options::new_json().from_json_slice_plain_return_borrowed(&json).unwrap();
    /// assert_eq!(modes, [
    ///    TravelMode { mode: "foot".to_string(), benefit: Some("healthy".to_string()) },
    ///    TravelMode { mode: "".to_string(), benefit: None },
    ///    // Note: this function fails on incomplete strings, because
    ///    // the randomized trailer is needed for those.
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
    /// **Note: This API is relatively likely to change (more unstable) compared to [`Self::from_json_str`].**
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
    /// let options = deser_incomplete::Options::new_json();
    /// let prepared = options.prepare_str_for_borrowed_deserialization(json.into());
    /// let modes: Vec<TravelMode> = options.from_json_str_borrowed(&prepared).unwrap();
    /// assert_eq!(modes, [
    ///    TravelMode { mode: "foot", benefit: Some("healthy") },
    ///    TravelMode { mode: "aeropl", benefit: None }
    /// ]);
    /// ```
    #[cfg(feature = "serde_json")]
    pub fn from_json_str_borrowed<'de, T>(
        self,
        InputPlusTrailer(prepared_json): &'de InputPlusTrailer<impl AsRef<str>>,
    ) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(source::JsonStr(prepared_json.as_ref()))
    }

    /// See [`Self::from_json_str_borrowed`].
    ///
    /// **Note: This API is relatively likely to change (more unstable) compared to [`Self::from_json_slice`].**
    #[cfg(feature = "serde_json")]
    pub fn from_json_slice_borrowed<'de, T>(
        self,
        InputPlusTrailer(prepared_json): &'de InputPlusTrailer<impl AsRef<[u8]>>,
    ) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(source::JsonBytes(prepared_json.as_ref()))
    }

    #[cfg(feature = "serde_yaml")]
    pub fn from_yaml_str_borrowed<'de, T>(
        self,
        InputPlusTrailer(prepared_yaml): &'de InputPlusTrailer<impl AsRef<str>>,
    ) -> Result<T, Error<serde_yaml::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(source::YamlStr(prepared_yaml.as_ref()))
    }

    #[cfg(feature = "serde_yaml")]
    pub fn from_yaml_slice_borrowed<'de, T>(
        self,
        InputPlusTrailer(prepared_yaml): &'de InputPlusTrailer<impl AsRef<[u8]>>,
    ) -> Result<T, Error<serde_yaml::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(source::YamlBytes(prepared_yaml.as_ref()))
    }

    /// Prepare a string for borrowed deserialization with a method like [`Self::from_json_str_borrowed`].
    ///
    /// This appends to the input, according to the randomized trailer method. And this returns a newtype
    /// wrapper, so you can undo the effects yourself.
    #[cfg(feature = "rand")]
    pub fn prepare_str_for_borrowed_deserialization<'a>(
        &self,
        mut input: Cow<'a, str>,
    ) -> InputPlusTrailer<Cow<'a, str>> {
        use options_impl::RandomTrailer as _;

        #[cfg(feature = "rand")]
        if let Some(tag) = self.random_tag.as_ref() {
            self.extra
                .get_random_trailer()
                .prepare_string_with_tag(Cow::to_mut(&mut input), tag);
        }
        InputPlusTrailer(input)
    }

    /// Prepare a slice for borrowed deserialization with a method like [`Self::from_json_slice_borrowed`].
    ///
    /// This appends to the input, according to the randomized trailer method. And this returns a newtype
    /// wrapper, so you can undo the effects yourself.
    #[cfg(feature = "rand")]
    pub fn prepare_slice_for_borrowed_deserialization<'a>(
        &self,
        mut input: Cow<'a, [u8]>,
    ) -> InputPlusTrailer<Cow<'a, [u8]>> {
        use options_impl::RandomTrailer as _;

        #[cfg(feature = "rand")]
        if let Some(tag) = self.random_tag.as_ref() {
            self.extra
                .get_random_trailer()
                .prepare_vec_with_tag(Cow::to_mut(&mut input), tag);
        }
        InputPlusTrailer(input)
    }

    #[cfg(feature = "unstable")]
    pub fn custom_behavior(self, behavior: UnstableCustomBehavior) -> Self {
        Options { behavior, ..self }
    }

    /// Don't use a random tag. This can make deserialization a tiny bit cheaper,
    /// because the input does not have to be reallocated.
    #[cfg(feature = "rand")]
    pub fn disable_random_tag(mut self) -> Self {
        self.random_tag = None;
        self
    }
}

#[cfg(feature = "rand")]
impl<R, F, RT> Options<ExtraOptionsStruct<R, F, RT>>
where
    R: options_impl::MakeReporter,
    F: options_impl::MakeFallbackProvider,
    RT: options_impl::RandomTrailer,
{
    /// Set a different method for randomized trailers.
    pub fn set_random_trailer<RT2>(
        self,
        random_trailer: RT2,
    ) -> Options<ExtraOptionsStruct<R, F, RT2>>
    where
        RT2: options_impl::RandomTrailer,
    {
        let Options {
            random_tag,
            max_n_backtracks,
            behavior,
            extra,
        } = self;

        Options {
            random_tag,
            max_n_backtracks,
            behavior,
            extra: ExtraOptionsStruct {
                make_reporter: extra.make_reporter,
                make_fallback_provider: extra.make_fallback_provider,
                random_trailer,
            },
        }
    }
}
