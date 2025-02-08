#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(
    not(all(feature = "rand", feature = "tracing")),
    allow(unused_variables, unused_imports, dead_code, unused_mut)
)]

//! # Deserialize incomplete or broken data with Serde
//!
//! Wrap Serde [`Deserializer`]s (like serde_json and serde_yaml) so you
//! can parse incomplete or tolerate broken data.
//!
//! Streaming JSON for instance is technically invalid until the stream is done.
//! But by tolerating premature end of input, we can do something useful while
//! the stream is in progress.
//!
//! <img src="https://bgeron.github.io/partial-deser/assets/live-travel-modes.gif" alt='Someone is slowly
//! typing JSON into a terminal program. The JSON is an array of objects.
//! The program gradually renders the JSON input as Rust debug output, and as a table.
//! The fields of the Rust struct are printed even though they are missing in the JSON input.
//! The example program is called "live".' title="Demo that shows parsing JSON as it is typed by the user"
//! style="max-height: 300px; height: auto; width: auto;">
//!
//! Here, we printed the Rust debug representation. We also reserialized to JSON and
//! let nushell do its beautiful table formatting.
//!
//! The JSON can also come from an external program. Here is a demo program that
//! computes disk usage of directories and outputs the results as JSON.
//! In true Unix style,  displaying for the user is a separate concern,
//! implemented by a separate program.
//!
//! <img src="https://bgeron.github.io/partial-deser/assets/du-live.gif" alt='A Unix pipeline with
//! two programs is shown. The source program computes the disk size
//! of a bunch of directories and outputs a JSON array of objects. The sink program
//! pretty-prints the JSON table. Computing the disk size takes a while, and you can
//! see which directory is being analyzed because the result for that directory is empty
//! while it is computing.' title='Demo that shows parsing JSON as it is generated live from another program that mimics du'
//! style="max-height: 350px; height: auto; width: auto;">
//!
//! `deser-incomplete` sits between `#[serde(Deserialize)]` and the data format, and
//! safely halts parsing on parse errors or other errors.
//!
//! <img src="https://bgeron.github.io/partial-deser/assets/deser-incomplete-blocks-errors.png" alt='This library sits
//! in between Deserialize and Deserializer. Information about the parsed data is successfully
//! sent from Deserializer through deser-incomplete to Deserialize. But errors from Deserializer are
//! blocked.' style="max-height: 250px; height: auto; width: auto;">
//!
//! ## How to use: JSON and YAML
//!
//! ```
//! let result: Result<Vec<u32>, deser_incomplete::Error<serde_json::Error>>
//!     = deser_incomplete::from_json_str("[3, 4, ");
//!
//! assert_eq!(result.unwrap(), vec![3, 4]);
//!
//! let result: Result<Vec<bool>, deser_incomplete::Error<serde_yaml::Error>>
//!    = deser_incomplete::from_yaml_str("- true\n- false\n- ");
//!
//! assert_eq!(result.unwrap(), vec![true, false]);
//! ```
//!
//! Command line:
//!
//! ```sh
//! $ cargo install deser-incomplete --example repair-deser
//!
//! $ echo '[3, 4' | repair-deser    # JSON by default
//! [3,4]
//! ```
//!
//! ## How to use: other data formats
//!
//! - You need to explain how to create the [`Deserializer`] by implementing [`Source`].
//!
//!   - If your format has `&mut T: Deserializer` then mimic [`source::JsonStr`].
//!   - If your format has `T: Deserializer` then mimic [`source::YamlStr`].
//!
//! - Some formats need a trailer for best results. For example, [`from_json_str`] appends
//!   a double-quote to the input before parsing, this lets [`serde_json`] see strings that weren't
//!   actually complete.
//!
//!   We also preprocess the input in [`from_yaml_str`], actually there it is even more important
//!   for good results.
//!
//!   _Add preprocessing with [`Options::set_random_trailer`], or turn it off such preprocessing
//!   with [`Options::disable_random_tag`]. You can see the effect of it with
//!   `cargo run --example live -- --use-random-trailer false`._
//!
//!   I expect that binary formats don't need this preprocessing.
//!
//!
//! ## How this works internally
//!
//! The implementation sits in between [`Deserialize`], [`Deserializer`], and [`Visitor`],
//! gathers metadata during the parse, and saves successful sub-parses. It also "backtracks":
//! if a parse fails, then we retry, but just before the failure point we swap out the real
//! [`Deserializer`] for a decoy which can brings deserialization to a safe end.
//!
//!
//! We apply multiple techniques. Suppose we want to parse `Vec<u32>` with [`serde_json`].
//! Here are the main techniques.
//!
//! 1. **(Example: parse empty JSON as `[]` .)** — On the top level, if parsing fails immediately (e.g.
//!    empty input) but a sequence is expected, then return `[]`.
//!
//!    _\[setting name: fallback_seq_empty_at_root]_
//!
//! 2. **(Example: parse JSON `"[3"` as `[3]` .)** — When there are no more elements in a sequence,
//!    let the [`Visitor`] construct the `Vec<u32>` and put it somewhere safe. Now
//!    `serde_json::Deserializer::deserialize_seq` notices the missing close bracket and
//!    returns `Err` to us. We ignore `Err`, retrieve the saved value again, and return `Ok`
//!    of it.
//!
//!    This happens for every `deserialize_*` method, not just sequences.
//!
//!    _\[setting name: tolerate_deserializer_fail_after_visit_success]_
//!
//! 3. **(Example: parse JSON `"[3,"` as `[3]` .)** — Inside a sequence, if parsing the next element will
//!    fail, then don't even try.
//!
//!    This works using backtracking.
//!
//!    _\[setting name: backtrack_seq_skip_item]_
//!
//! 4. Before deserializing, we append a random trailer.
//!
//! #### Random trailer
//!
//! Additionally we have a "random trailer" technique to get incomplete strings to parse.
//! Unfortunately this technique is specific to the data format. This library implements
//! it for JSON and YAML.
//!
//! This technique is not applied by default for other data formats. Even with JSON/YAML, this
//! technique can be turned off with [`Options::disable_random_tag`].
//!
//! #### Random trailer for JSON
//!
//! We actually [append][append-impl] `tRANDOM"` to every JSON input, where `RANDOM` are some randomly chosen
//! letters. It turns out that [`serde_json`] can parse any prefix of valid JSON, as long
//! as we concatenate `tRANDOM"` to it. Some examples:
//!
//! 1. **(Example: `"hello` .)** The concatenation is `"hellotRANDOM"` and we actually get
//!     this back from [`serde_json`] through `fn visit_borrowed_str` --- after [`serde_json`]
//!     removed the double-quotes.
//!
//!     In `fn visit_borrowed_str`, we notice that the string ends in `RANDOM`. Because this
//!     is a random string of letters, it cannot have been part of the incomplete JSON input.
//!     We remove the `tRANDOM` suffix and get back just `"hello"`.
//!
//! 2. **(Example: `"hello\` --- perhaps breaking in the middle of `\n` .)** The concatenation
//!     is `"hello\tRANDOM"`; the `\t` parses to a tab character. We strip off `<TAB>random`
//!     and again return `"hello"`.
//!
//! 3. **(Example: `"hello"` .)** The concatenation is `"hello"tRANDOM"`. Now [`serde_json`]
//!     visits the `hello` string as it would normally do, and if there should be any error
//!     after the visit, we can recover from it anyway as
//!     per _tolerate_deserializer_fail_after_visit_success_.
//!
//! [append-impl]: https://github.com/bgeron/partial-deser/blob/raw-unreleased/src/random_trailer/json.rs
//!
//! #### Inspecting at runtime
//!
//! There is extensive logging through the [`tracing`] library, which becomes visible if you
//! initialize the library.
//!
//! #### Guiding principles
//!
//! The logic was hand-tweaked to the following criteria:
//!
//! 1. ("soundness") For any complete and valid JSON/YAML, if you call `deser-incomplete`
//!    on a prefix, then its output should not contain data that doesn't exist in the
//!    complete JSON/YAML.
//!
//! 2. ("monotone") A larger prefix should not parse to a shorter output.
//!
//! 3. ("prompt") Ideally, each prefix contains as much data as we can be certain of.
//!
//! The implementation of [`Deserializer`] (data format) may influence the quality of the output,
//! but the default ruleset does generally very well with [`serde_json`] and [`serde_yaml`].
//!
//! There are [extensive snapshot tests][snapshot-tests] that validate the quality of the output
//! on these criteria.
//!
//! If you are curious, then it is possible to tweak the ruleset
//! with `unstable::UnstableCustomBehavior`. We also have snapshot tests for some alternative
//! parsing configurations.
//!
//! [snapshot-tests]: https://github.com/bgeron/partial-deser/blob/raw-unreleased/tests/output/json_output/seq.rs
//!
//! ## Notes and limitations
//!
//! - Ideally, your data format should be relatively greedy, in the sense that it
//!   generates information quickly and does not need to look ahead in the serialized
//!   stream too much.
//!
//! - This approach lets us safely abort parsing and get a value, but
//!   we cannot skip over invalid segments of input. (For that you need
//!   an approach like [tree-sitter](https://tree-sitter.github.io/).)
//!
//! - We cannot distinguish eof from invalid input.
//!
//! - YAML works well in general, but it is a bit less exhaustively tested than JSON.
//!   The randomized trailer is really important for YAML.
//!
//! - JSON: when parsing a floating-point number, if the end of input happens to fall
//!   directly after the decimal point, then the number is missing from the output.
//!
//! - For YAML, the randomized trailer uses a heuristic to see if we are currently in
//!   an escape sequence in a string --- but this heuristic can fail. In this case,
//!   the incomplete string will be missing from the output.
//!
//!
//! Have fun!

use std::borrow::Cow;
#[cfg(feature = "rand")]
use std::sync::Arc;

use options::DefaultExtraOptions;
#[cfg(doc)]
use serde::{de::Visitor, Deserialize, Deserializer};

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
pub mod error;
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

/// Types and traits that have to be public to satisfy rustc/rustdoc.
///
/// Instead of looking here, look at the methods of [`crate::Options`].
pub mod options {
    #[cfg(all(feature = "rand", feature = "serde_json"))]
    pub use crate::options_impl::JsonExtraOptions;
    #[cfg(all(feature = "rand", feature = "serde_yaml"))]
    pub use crate::options_impl::YamlExtraOptions;
    pub use crate::options_impl::{
        DefaultExtraOptions, ExtraOptions, MakeDefaultFallbacks, MakeDefaultReporter,
    };
}

/// Import from this crate in this library. That way, doc links work properly.
#[cfg(not(feature = "unstable"))]
mod unstable {
    pub use crate::collection_of_unstable_stuff::*;
}
/// Stuff that is not polished or likely to change.
#[cfg(feature = "unstable")]
pub mod unstable {
    pub use crate::collection_of_unstable_stuff::*;
}

#[allow(unused_imports)]
mod collection_of_unstable_stuff {
    pub use crate::fallback::Fallbacks;
    pub use crate::options_impl::{
        ExtraOptions, ExtraOptionsStruct, MakeFallbackProvider, MakeReporter,
        UnstableCustomBehavior,
    };
    pub use crate::reporter::{DefaultReporter, Reporter};
    pub trait ExtraOptionsIsUnstable {}
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
    /// This comes at the cost that we cannot use the random trailer technique that gives
    /// us access to the contents of incomplete strings.
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
    /// the random trailer that gives us access to the contents of incomplete strings.
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
