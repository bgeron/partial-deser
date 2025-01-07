use partial_deser::Options;
use serde::{Deserialize, Serialize};

use crate::common::{run_on_prefixes_and_format_outputs, ComparisonLine};
use crate::print_as_constructor::PrintAsConstructor;

mod any;
mod bool;

/// Partially deserialize all prefixes of the input as JSON. Reserialize the successful
/// results to JSON for comparison with `assert_eq!`, and stringify any errors.
///
/// The output is deduplicated -- only inputs are shown where the output changes.
#[allow(clippy::type_complexity)]
pub(crate) fn run_json_modes_on_prefixes_and_format_outputs<'input, T: for<'de> Deserialize<'de> + Serialize>(
    modes: &[(&'static str, Options)],
    full_input: &'input [u8],
) -> PrintAsConstructor<Vec<Vec<ComparisonLine<'input, Result<serde_json::Value, String>>>>> {
    use itertools::Itertools;

    modes
        .iter()
        .map(|(mode_desc, options)| {
            let outputs = run_on_prefixes_and_format_outputs(full_input, |inp| {
                options
                    .clone()
                    .from_json_slice::<T>(inp)
                    .map(|value| serde_json::to_value(&value).expect("could not reserialize to JSON"))
                    .map_err(|err| err.to_string())
            });

            [ComparisonLine::Heading(&mode_desc)].into_iter().chain(outputs).collect_vec()
        })
        .collect_vec()
        .into()
}

fn default_modes() -> Vec<(&'static str, Options)> {
    vec![
        ("default behavior", Options::new_json()),
        ("default behavior, 0 backtracks", Options::new_json().with_max_n_backtracks(Some(0))),
        ("default behavior, 1 backtracks", Options::new_json().with_max_n_backtracks(Some(1))),
        (
            "strict behavior",
            Options::new_json().custom_behavior(partial_deser::unstable::UnstableCustomBehavior::strict()),
        ),
    ]
}
