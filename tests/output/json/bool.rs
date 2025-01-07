use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};
use crate::print_as_constructor::prelude::*;

#[test]
fn test_bools() {
    assert_eq!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<bool>>(&default_modes(), b"[true, false, true]"),
        // Vec::<Vec<crate::json::ComparisonLine<Result<serde_json::Value, String>>>>::new()
        vec![
            vec![
                Heading("default behavior"),
                Input(B(b"")),
                Output(Ok(json!([]))),
                Input(B(b"[true")),
                Output(Ok(json!([true]))),
                Input(B(b"[true, false")),
                Output(Ok(json!([true, false]))),
                Input(B(b"[true, false, true")),
                Output(Ok(json!([true, false, true])))
            ],
            vec![
                Heading("default behavior, 0 backtracks"),
                Input(B(b"")),
                Output(Ok(json!([]))),
                Input(B(b"[true")),
                Output(Ok(json!([true]))),
                Input(B(b"[true, false")),
                Output(Ok(json!([true, false]))),
                Input(B(b"[true, false, true")),
                Output(Ok(json!([true, false, true])))
            ],
            vec![
                Heading("default behavior, 1 backtracks"),
                Input(B(b"")),
                Output(Ok(json!([]))),
                Input(B(b"[true")),
                Output(Ok(json!([true]))),
                Input(B(b"[true, false")),
                Output(Ok(json!([true, false]))),
                Input(B(b"[true, false, true")),
                Output(Ok(json!([true, false, true])))
            ],
            vec![
                Heading("strict behavior"),
                Input(B(b"")),
                Output(Ok(json!([]))),
                Input(B(b"[")),
                Output(Err(
                    "could not find a potential backtrack point (do you have #[serde(default)] on your top-level type?) (after 0 backtracks)"
                        .to_string()
                )),
                Input(B(b"[true, false, true]")),
                Output(Ok(json!([true, false, true])))
            ]
        ]
    );
}
