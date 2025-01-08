use std::borrow::Cow;

use bstr::BStr;
use indexmap::IndexMap;
use serde::Serialize;

/// Run function on all prefixes of the input.
///
/// The output is deduplicated -- only inputs are shown where the output changes.
pub(crate) fn run_on_prefixes_and_format_outputs<Output: PartialEq>(full_input: &[u8], f: impl Fn(&[u8]) -> Output) -> IndexMap<Cow<str>, Output> {
    let mut outputs: Vec<(&[u8], Output)> = Vec::new();

    for end in 0..=full_input.len() {
        let range = &full_input[..end];
        let output = f(range);

        if Some(&output) != outputs.last().map(|(_, out)| out) {
            outputs.push((range, output));
        }
    }

    outputs.into_iter().map(|(input, output)| (String::from_utf8_lossy(input), output)).collect()
}
