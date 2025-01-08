use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_unit() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<()>>(&default_modes(), &r#"[null, null]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[n": Ok([
          (),
        ]),
        "[null, n": Ok([
          (),
          (),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[n": Ok([
          (),
        ]),
        "[null, n": Ok([
          (),
          (),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, null]": Ok([
          (),
          (),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type?) (after 0 backtracks)"),
        "[": Ok([]),
        "[null": Ok([
          (),
        ]),
        "[null, null": Ok([
          (),
          (),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[n": Ok([
          (),
        ]),
        "[null, n": Ok([
          (),
          (),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type?) (after 0 backtracks)"),
        "[null, null]": Ok([
          (),
          (),
        ]),
      },
    }
    "###
    );
}
