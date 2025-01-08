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
    struct Tuple(Vec<bool>, Vec<bool>, Vec<bool>);

    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Tuple>>(&default_modes(), b"[[[true], [false], [true]], [[false], [true], [false]]]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[[true], [false], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[[[true], [false], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[[true], [false], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
    }
    "###)
}

#[test]
fn test_tuple_struct_with_default() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Tuple(
        Vec<bool>,
        #[serde(default)] Vec<bool>,
        #[serde(default)] Vec<bool>,
    );

    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Tuple>>(&default_modes(), b"[[[true], [false], [true]], [[false], [true], [false]]]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[[": Ok([
          Tuple([], [], []),
        ]),
        "[[[true": Ok([
          Tuple([
            true,
          ], [], []),
        ]),
        "[[[true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([], [], []),
        ]),
        "[[[true], [false], [true]], [[false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [], []),
        ]),
        "[[[true], [false], [true]], [[false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[[[": Ok([
          Tuple([], [], []),
        ]),
        "[[[true": Ok([
          Tuple([
            true,
          ], [], []),
        ]),
        "[[[true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([], [], []),
        ]),
        "[[[true], [false], [true]], [[false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [], []),
        ]),
        "[[[true], [false], [true]], [[false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[": Ok([
          Tuple([], [], []),
        ]),
        "[[[true": Ok([
          Tuple([
            true,
          ], [], []),
        ]),
        "[[[true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([], [], []),
        ]),
        "[[[true], [false], [true]], [[false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [], []),
        ]),
        "[[[true], [false], [true]], [[false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[[": Ok([
          Tuple([], [], []),
        ]),
        "[[[true": Ok([
          Tuple([
            true,
          ], [], []),
        ]),
        "[[[true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([], [], []),
        ]),
        "[[[true], [false], [true]], [[false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [], []),
        ]),
        "[[[true], [false], [true]], [[false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
    }
    "###)
}
