use std::fmt::{Display, Formatter, Write as _};
use std::io::Write as _;

use crate::string_like::StringLike;

/// A prepared input for deserialization with the JSON string trick.
///
/// **Note: This API is relatively likely to change (unstable).**
pub struct Prepared<SliceType>(pub SliceType);

pub(crate) fn prepare_string_with_tag(tag: &str, input: &mut String) {
    write!(input, "{}", TagSuffix { tag }).expect("writing to a string always succeeds")
}

pub(crate) fn prepare_vec_with_tag(tag: &str, input: &mut Vec<u8>) {
    write!(input, "{}", TagSuffix { tag }).expect("writing to a vec always succeeds")
}

/// Prints as whatever we suffix an incomplete JSON input with, before passing it
/// through [`serde_json`], to ensure that we actually receive this unfinished string
/// from [`serde_json`] and it won't give up on this string  when it doesn't see the
/// terminating double-quote.
///
/// For this, we want
///
///   - a double-quote, to terminate such a string,
///   - an letter like `t` that forms a legal escape sequence, so that if the
///     input ends with `\`, with the suffix it'll be `...\t..."` and our double-quote
///     won't be escaped.
///   - a random tag (e.g. `BERLIN`). So if [`serde_json`] visits us with a
///     string `Hello, wotBERLIN`, then we know that only `Hello, wo` was actually part
///     of the input.
struct TagSuffix<'a> {
    tag: &'a str,
}

impl Display for TagSuffix<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"t{}""#, self.tag)
    }
}

/// Take off the tag suffix from a deserialized string. Report whether the suffix was present.
#[must_use]
pub(crate) fn undo_tag_suffix(stringy: &mut impl StringLike, tag: &str) -> bool {
    if stringy.ends_with_string(tag) {
        let target_len = stringy.len()
            - tag.len()
            // Both `"t"` and `"\t"` are 1 byte
            - 1;

        stringy.truncate_to_bytes(target_len);
        true
    } else {
        false
    }
}

#[cfg(test)]
#[path = ""]
mod test {

    #[path = "../tests/output/json_output/cow_string.rs"]
    mod cow_string;

    use cow_string::CowString;
    use serde::Deserialize;

    use super::*;

    const TAG: &str = "BERLIN";

    fn parse_and_undo_tag(prepared: &str) -> (CowString<'_>, bool) {
        let mut parsed: CowString =
            CowString::deserialize(&mut serde_json::Deserializer::from_str(prepared)).unwrap();

        let had_tag = match &mut parsed {
            CowString::VisitBorrowedStr(s) => undo_tag_suffix(s, TAG),
            CowString::VisitStr { cloned } => undo_tag_suffix(cloned, TAG),
            CowString::VisitString(s) => undo_tag_suffix(s, TAG),
        };

        (parsed, had_tag)
    }

    #[test]
    fn test_tag_suffix() {
        for (input, will_encounter_end, expected_result) in [
            (
                r#""Hello, wo"#,
                true,
                CowString::VisitBorrowedStr("Hello, wo"),
            ),
            (
                r#""Hello, wo\"#,
                true,
                // Unfinished backslash escape in JSON -> we can't know what it'll eventually represent.
                //
                // serde_json has to convert "\t" into <TAB>, so the parsed JSON is no longer a substring
                // of the JSON itself -> CowString::Owned.
                CowString::VisitStr {
                    cloned: "Hello, wo".to_string(),
                },
            ),
            (
                r#""Hello, world""#,
                false,
                CowString::VisitBorrowedStr("Hello, world"),
            ),
            (
                // \n converts into newline
                r#""Hello,\nworld""#,
                false,
                CowString::VisitStr {
                    cloned: "Hello,\nworld".to_string(),
                },
            ),
        ] {
            let mut prepared = input.to_string();
            prepare_string_with_tag(TAG, &mut prepared);
            let (result, encountered_end): (CowString, bool) = parse_and_undo_tag(&prepared);
            assert_eq!(result, expected_result, "input = {input:?}");
            assert_eq!(encountered_end, will_encounter_end, "input = {input:?}");
            assert_eq!(
                matches!(result, CowString::VisitBorrowedStr(_)),
                matches!(expected_result, CowString::VisitBorrowedStr(_)),
                "input = {input:?}"
            )
        }
    }
}
