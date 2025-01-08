use serde::{Deserialize, Serialize};

use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_unit_struct() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Unit;

    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Unit>>(&default_modes(), b"[null, null]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[n": Ok([
          Unit,
        ]),
        "[null, n": Ok([
          Unit,
          Unit,
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[n": Ok([
          Unit,
        ]),
        "[null, n": Ok([
          Unit,
          Unit,
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, null]": Ok([
          Unit,
          Unit,
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[null": Ok([
          Unit,
        ]),
        "[null, null": Ok([
          Unit,
          Unit,
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[n": Ok([
          Unit,
        ]),
        "[null, n": Ok([
          Unit,
          Unit,
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[null, null]": Ok([
          Unit,
          Unit,
        ]),
      },
    }
    "###
    );
}


#[test]
fn test_newtype_struct() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Newtype(u8);

    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Unit>>(&default_modes(), b"[42]"),
        @"")}
