use std::borrow::Cow;
use std::fmt::Debug;

use indexmap::IndexMap;
use partial_deser::unstable::UnstableCustomBehavior;
use partial_deser::Options;
use serde::{Deserialize, Serialize};

use crate::common::run_on_prefixes_and_format_outputs;

mod any;
mod bool;
mod number;
mod other;
mod seq;

/// Partially deserialize all prefixes of the input as JSON. Reserialize the successful
/// results to JSON for comparison with `assert_eq!`, and stringify any errors.
///
/// The output is deduplicated -- only inputs are shown where the output changes.
#[allow(clippy::type_complexity)]
pub(crate) fn run_json_modes_on_prefixes_and_format_outputs<
    'input,
    T: for<'de> Deserialize<'de> + Serialize + Debug + PartialEq,
>(
    modes: &[(&'static str, Options)],
    full_input: &'input impl AsRef<[u8]>,
) -> IndexMap<&'input str, IndexMap<Cow<'input, str>, Result<impl Serialize, String>>> {
    let full_input = full_input.as_ref();

    modes
        .iter()
        .map(|(mode_desc, options)| {
            let outputs = run_on_prefixes_and_format_outputs(full_input, |inp| {
                options
                    .clone()
                    .from_json_slice::<T>(inp)
                    .map_err(|err| err.to_string())
            });

            (*mode_desc, outputs)
        })
        .collect()
}

fn default_modes() -> Vec<(&'static str, Options)> {
    vec![
        ("default behavior", Options::new_json()),
        (
            "default behavior, 0 backtracks",
            Options::new_json().with_max_n_backtracks(Some(0)),
        ),
        (
            "no fallbacks, 0 backtracks",
            Options::new_json()
                .custom_behavior(UnstableCustomBehavior::default().no_fallbacks())
                .with_max_n_backtracks(Some(0)),
        ),
        (
            "no fallbacks, 1 backtracks",
            Options::new_json()
                .custom_behavior(UnstableCustomBehavior::default().no_fallbacks())
                .with_max_n_backtracks(Some(1)),
        ),
        (
            "default behavior, 1 backtracks",
            Options::new_json().with_max_n_backtracks(Some(1)),
        ),
        (
            "strict behavior",
            Options::new_json().custom_behavior(UnstableCustomBehavior::strict()),
        ),
    ]
}
