use serde::{Deserialize, Serialize};

use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Struct {
    x: Vec<bool>,
    y: Vec<bool>,
    z: Vec<bool>,
}

#[test]
fn test_toplevel_struct() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Struct>(&default_modes(), &r#"{"x": [true], "y": [false], "z": [true]}"#),
        @r###"
    {
      "default behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "{\"x\": [true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "{\"x\": [true], \"y\": [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\":": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior except no JSON-specific tricks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "{\"x\": [true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "{\"x\": [true], \"y\": [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\":": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\":": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]}": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\":": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]}": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
    }
    "###)
}

#[test]
fn test_struct() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Struct>>(&default_modes(), &r#"[{"x": [true], "y": [false], "z": [true]}, {"x": [false], "y": [true], "z": [false]}]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\":": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\":": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "default behavior except no JSON-specific tricks": {
        "": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\":": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\":": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\":": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\":": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\":": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\":": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
    }
    "###)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct StructWithDefault {
    x: Vec<bool>,
    #[serde(default)]
    y: Vec<bool>,
    #[serde(default)]
    z: Vec<bool>,
}

#[test]
fn test_toplevel_struct_with_default() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<StructWithDefault>(&default_modes(), &r#"{"x": [true], "y": [false], "z": [true]}"#),
        @r###"
    {
      "default behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\":": Ok(StructWithDefault(
          x: [],
          y: [],
          z: [],
        )),
        "{\"x\": [true": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\":": Ok(StructWithDefault(
          x: [],
          y: [],
          z: [],
        )),
        "{\"x\": [true": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\"": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\":": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\"": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\":": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]}": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [": Ok(StructWithDefault(
          x: [],
          y: [],
          z: [],
        )),
        "{\"x\": [true": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\":": Ok(StructWithDefault(
          x: [],
          y: [],
          z: [],
        )),
        "{\"x\": [true": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]}": Ok(StructWithDefault(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
    }
    "###)
}

#[test]
fn test_struct_with_default() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<StructWithDefault>>(&default_modes(), &r#"[{"x": [true], "y": [false], "z": [true]}, {"x": [false], "y": [true], "z": [false]}])"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[{\"x\":": Ok([
          StructWithDefault(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\"": Ok([]),
        "[{\"x\": [true], \"y\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\"": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\"": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[{\"x\":": Ok([
          StructWithDefault(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\"": Ok([]),
        "[{\"x\": [true], \"y\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\"": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\"": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [": Ok([
          StructWithDefault(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[{\"x\":": Ok([
          StructWithDefault(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\"": Ok([]),
        "[{\"x\": [true], \"y\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\"": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\"": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\":": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          StructWithDefault(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithDefault(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
    }
    "###)
}
