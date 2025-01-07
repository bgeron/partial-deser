use crate::print_as_constructor::{FmtConstructor, PrintAsConstructorByRef};

/// Run function on all prefixes of the input.
///
/// The output is deduplicated -- only inputs are shown where the output changes.
pub(crate) fn run_on_prefixes_and_format_outputs<Output: PartialEq>(full_input: &[u8], f: impl Fn(&[u8]) -> Output) -> Vec<ComparisonLine<Output>> {
    let mut outputs: Vec<(&[u8], Output)> = Vec::new();

    for end in 0..=full_input.len() {
        let range = &full_input[..end];
        let output = f(range);

        if Some(&output) != outputs.last().map(|(_, out)| out) {
            outputs.push((range, output));
        }
    }

    outputs
        .into_iter()
        .flat_map(|(input, output)| [ComparisonLine::Input(input), ComparisonLine::Output(output)])
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ComparisonLine<'a, Output, Input = &'a [u8]> {
    Heading(&'a str),
    Input(Input),
    Output(Output),
}

impl<Output, Input> FmtConstructor for ComparisonLine<'_, Output, Input>
where
    Input: FmtConstructor,
    Output: FmtConstructor,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonLine::Heading(heading) => {
                write!(f, "Heading({:?})", heading)
            }
            ComparisonLine::Input(input) => {
                write!(f, "Input({})", PrintAsConstructorByRef(input))
            }
            ComparisonLine::Output(output) => {
                write!(f, "Output({})", PrintAsConstructorByRef(output))
            }
        }
    }
}
