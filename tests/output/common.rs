use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::print_as_constructor::PrintAsConstructor;

/// Partially deserialize all prefixes of the input as JSON. Reserialize the successful
/// results to JSON for comparison with `assert_eq!`, and stringify any errors.
///
/// The output is deduplicated -- only inputs are shown where the output changes.
#[cfg(feature = "serde_json")]
#[allow(clippy::type_complexity)]
pub(crate) fn run_json_on_prefixes_and_format_outputs<T: for<'de> Deserialize<'de> + Serialize>(
    full_input: &[u8],
) -> PrintAsConstructor<Vec<(&[u8], Result<serde_json::Value, String>)>> {
    run_on_prefixes_and_format_outputs(full_input, |inp| {
        partial_deser::from_json_slice::<T>(inp)
            .map(|value| serde_json::to_value(&value).expect("could not reserialize to JSON"))
            .map_err(|err| err.to_string())
    })
}

/// Run function on all prefixes of the input.
///
/// The output is deduplicated -- only inputs are shown where the output changes.
pub(crate) fn run_on_prefixes_and_format_outputs<Output: PartialEq>(
    full_input: &[u8],
    f: impl Fn(&[u8]) -> Output,
) -> PrintAsConstructor<Vec<(&[u8], Output)>> {
    let mut outputs: Vec<(&[u8], Output)> = Vec::new();

    for end in 0..=full_input.len() {
        let range = &full_input[..end];
        let output = f(range);

        if Some(&output) != outputs.last().map(|(_, out)| out) {
            outputs.push((range, output));
        }
    }

    PrintAsConstructor(outputs)
}

pub(crate) fn stringify_output_errors<In, T, E: Display>(
    results: impl IntoIterator<Item = (In, Result<T, E>)>,
) -> Vec<(In, Result<T, String>)> {
    results
        .into_iter()
        .map(|(input, result)| {
            let output = match result {
                Ok(value) => Ok(value),
                Err(err) => Err(err.to_string()),
            };

            (input, output)
        })
        .collect()
}
