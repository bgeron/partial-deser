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
    struct Newtype(Vec<()>);

    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Newtype>>(&default_modes(), b"[[], [null, null], []]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [n": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null, n": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
        ]),
        "[[], [null, null], [": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [n": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null, n": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
        ]),
        "[[], [null, null], [": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[], [null, null], []]": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [null": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null, null": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
        ]),
        "[[], [null, null], [": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [n": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null, n": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
        ]),
        "[[], [null, null], [": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[], [null, null], []]": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
    }
    "###)
}

#[test]
fn test_tuple_struct() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Tuple(u8, u8, u8);

    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Tuple>>(&default_modes(), b"[[12, 34, 56], [12, 34, 56]]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [12, 34, 56]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[12, 34, 56], [12, 34, 56]]": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[12, 34, 56], [12, 34, 56]]": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
    }
    "###)
}

#[test]
fn test_tuple_struct_with_default() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Tuple(u8, #[serde(default)] u8, #[serde(default)] u8);

    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Tuple>>(&default_modes(), b"[[12, 34, 56], [12, 34, 56]]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[1": Ok([
          Tuple(1, 0, 0),
        ]),
        "[[12": Ok([
          Tuple(12, 0, 0),
        ]),
        "[[12, 3": Ok([
          Tuple(12, 3, 0),
        ]),
        "[[12, 34": Ok([
          Tuple(12, 34, 0),
        ]),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [1": Ok([
          Tuple(12, 34, 56),
          Tuple(1, 0, 0),
        ]),
        "[[12, 34, 56], [12": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 0, 0),
        ]),
        "[[12, 34, 56], [12, 3": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 3, 0),
        ]),
        "[[12, 34, 56], [12, 34": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 0),
        ]),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[[1": Ok([
          Tuple(1, 0, 0),
        ]),
        "[[12": Ok([
          Tuple(12, 0, 0),
        ]),
        "[[12, 3": Ok([
          Tuple(12, 3, 0),
        ]),
        "[[12, 34": Ok([
          Tuple(12, 34, 0),
        ]),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [1": Ok([
          Tuple(12, 34, 56),
          Tuple(1, 0, 0),
        ]),
        "[[12, 34, 56], [12": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 0, 0),
        ]),
        "[[12, 34, 56], [12, 3": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 3, 0),
        ]),
        "[[12, 34, 56], [12, 34": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 0),
        ]),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [12, 34, 56]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[12, 34, 56], [12, 34, 56]]": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[1": Ok([
          Tuple(1, 0, 0),
        ]),
        "[[12": Ok([
          Tuple(12, 0, 0),
        ]),
        "[[12, 3": Ok([
          Tuple(12, 3, 0),
        ]),
        "[[12, 34": Ok([
          Tuple(12, 34, 0),
        ]),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[12, 34, 56], [1": Ok([
          Tuple(12, 34, 56),
          Tuple(1, 0, 0),
        ]),
        "[[12, 34, 56], [12": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 0, 0),
        ]),
        "[[12, 34, 56], [12, 3": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 3, 0),
        ]),
        "[[12, 34, 56], [12, 34": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 0),
        ]),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[1": Ok([
          Tuple(1, 0, 0),
        ]),
        "[[12": Ok([
          Tuple(12, 0, 0),
        ]),
        "[[12, 3": Ok([
          Tuple(12, 3, 0),
        ]),
        "[[12, 34": Ok([
          Tuple(12, 34, 0),
        ]),
        "[[12, 34, 5": Ok([
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56": Ok([
          Tuple(12, 34, 56),
        ]),
        "[[12, 34, 56], [1": Ok([
          Tuple(12, 34, 56),
          Tuple(1, 0, 0),
        ]),
        "[[12, 34, 56], [12": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 0, 0),
        ]),
        "[[12, 34, 56], [12, 3": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 3, 0),
        ]),
        "[[12, 34, 56], [12, 34": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 0),
        ]),
        "[[12, 34, 56], [12, 34, 5": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 5),
        ]),
        "[[12, 34, 56], [12, 34, 56": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[12, 34, 56], [12, 34, 56]]": Ok([
          Tuple(12, 34, 56),
          Tuple(12, 34, 56),
        ]),
      },
    }
    "###)
}
