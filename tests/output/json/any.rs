use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};
use crate::print_as_constructor::prelude::*;

#[test]
fn test_any() {
    assert_eq!(
        run_json_modes_on_prefixes_and_format_outputs::<serde_json::Value>(&default_modes(), b"[true, false, 3, 4.5, \"hello\"]"),
        // Vec::<Vec<crate::json::ComparisonLine<Result<serde_json::Value, String>>>>::new()
        vec![
            vec![
                Heading("default behavior"),
                Input(B(b"")),
                Output(Err(
                    "could not find a potential backtrack point (do you have #[serde(default)] on your top-level type?) (after 0 backtracks)".to_string()
                )),
                Input(B(b"[true, false, 3, 4.5, \"hello\"]")),
                Output(Ok(json!([true, false, 3, 4.5, "hello"])))
            ],
            vec![
                Heading("default behavior, 0 backtracks"),
                Input(B(b"")),
                Output(Err(
                    "could not find a potential backtrack point (do you have #[serde(default)] on your top-level type?) (after 0 backtracks)".to_string()
                )),
                Input(B(b"[true, false, 3, 4.5, \"hello\"]")),
                Output(Ok(json!([true, false, 3, 4.5, "hello"])))
            ],
            vec![
                Heading("default behavior, 1 backtracks"),
                Input(B(b"")),
                Output(Err(
                    "could not find a potential backtrack point (do you have #[serde(default)] on your top-level type?) (after 0 backtracks)".to_string()
                )),
                Input(B(b"[true, false, 3, 4.5, \"hello\"]")),
                Output(Ok(json!([true, false, 3, 4.5, "hello"])))
            ],
            vec![
                Heading("strict behavior"),
                Input(B(b"")),
                Output(Err(
                    "could not find a potential backtrack point (do you have #[serde(default)] on your top-level type?) (after 0 backtracks)".to_string()
                )),
                Input(B(b"[true, false, 3, 4.5, \"hello\"]")),
                Output(Ok(json!([true, false, 3, 4.5, "hello"])))
            ]
        ]
    );
}
