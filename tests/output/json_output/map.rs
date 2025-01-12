use indexmap::IndexMap;

use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_toplevel_map() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<IndexMap<String, String>>(&
            default_modes(), &r#"{"abc": "def", "ghi": "jkl"}"#),
            @r###"
    {
      "default behavior": {
        "": Ok({}),
        "{\"abc\": \"": Ok({
          "abc": "",
        }),
        "{\"abc\": \"d": Ok({
          "abc": "d",
        }),
        "{\"abc\": \"de": Ok({
          "abc": "de",
        }),
        "{\"abc\": \"def": Ok({
          "abc": "def",
        }),
        "{\"abc\": \"def\", \"ghi\": \"": Ok({
          "abc": "def",
          "ghi": "",
        }),
        "{\"abc\": \"def\", \"ghi\": \"j": Ok({
          "abc": "def",
          "ghi": "j",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jk": Ok({
          "abc": "def",
          "ghi": "jk",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jkl": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "default behavior, 0 backtracks": {
        "": Ok({}),
        "{\"abc\"": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"abc\": \"": Ok({
          "abc": "",
        }),
        "{\"abc\": \"d": Ok({
          "abc": "d",
        }),
        "{\"abc\": \"de": Ok({
          "abc": "de",
        }),
        "{\"abc\": \"def": Ok({
          "abc": "def",
        }),
        "{\"abc\": \"def\", \"ghi\"": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"abc\": \"def\", \"ghi\": \"": Ok({
          "abc": "def",
          "ghi": "",
        }),
        "{\"abc\": \"def\", \"ghi\": \"j": Ok({
          "abc": "def",
          "ghi": "j",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jk": Ok({
          "abc": "def",
          "ghi": "jk",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jkl": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"abc\": \"def\", \"ghi\": \"jkl\"}": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Ok({}),
        "{\"abc\": \"": Ok({
          "abc": "",
        }),
        "{\"abc\": \"d": Ok({
          "abc": "d",
        }),
        "{\"abc\": \"de": Ok({
          "abc": "de",
        }),
        "{\"abc\": \"def": Ok({
          "abc": "def",
        }),
        "{\"abc\": \"def\", \"ghi\": \"": Ok({
          "abc": "def",
          "ghi": "",
        }),
        "{\"abc\": \"def\", \"ghi\": \"j": Ok({
          "abc": "def",
          "ghi": "j",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jk": Ok({
          "abc": "def",
          "ghi": "jk",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jkl": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "default behavior, 1 backtracks": {
        "": Ok({}),
        "{\"abc\": \"": Ok({
          "abc": "",
        }),
        "{\"abc\": \"d": Ok({
          "abc": "d",
        }),
        "{\"abc\": \"de": Ok({
          "abc": "de",
        }),
        "{\"abc\": \"def": Ok({
          "abc": "def",
        }),
        "{\"abc\": \"def\", \"ghi\": \"": Ok({
          "abc": "def",
          "ghi": "",
        }),
        "{\"abc\": \"def\", \"ghi\": \"j": Ok({
          "abc": "def",
          "ghi": "j",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jk": Ok({
          "abc": "def",
          "ghi": "jk",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jkl": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"abc\": \"def\", \"ghi\": \"jkl\"}": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
    }
    "###)
}

#[test]
fn test_map() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<IndexMap<String, String>>>(&
            default_modes(), &r#"[{"ab": "cd", "ef": "gh"}, {"AB": "CD", "EF": "GH"}]"#),
            @"")
}
