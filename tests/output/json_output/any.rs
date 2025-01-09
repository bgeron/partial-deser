use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_any() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<serde_json::Value>(&default_modes(), &"[true, false, 3, 4.5, \"hello\"]"),
        @r###"
    {
      "default behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[true, false, 3, 4.5, \"hello\"]": Ok([
          true,
          false,
          3,
          4.5,
          "hello",
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[true, false, 3, 4.5, \"hello\"]": Ok([
          true,
          false,
          3,
          4.5,
          "hello",
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[true, false, 3, 4.5, \"hello\"]": Ok([
          true,
          false,
          3,
          4.5,
          "hello",
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[true, false, 3, 4.5, \"hello\"]": Ok([
          true,
          false,
          3,
          4.5,
          "hello",
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[true, false, 3, 4.5, \"hello\"]": Ok([
          true,
          false,
          3,
          4.5,
          "hello",
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[true, false, 3, 4.5, \"hello\"]": Ok([
          true,
          false,
          3,
          4.5,
          "hello",
        ]),
      },
    }
    "###
    );
}
