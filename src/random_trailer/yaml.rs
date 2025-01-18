use std::fmt::{Display, Formatter, Write as _};
use std::io::Write as _;

use super::{RandomTrailer, StringLike};

struct TagSuffix<'a> {
    tag: &'a str,
}

impl Display for TagSuffix<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"t\n\n# {}'""#, self.tag)
    }
}

#[derive(Clone, Debug, Default)]
pub struct YamlRandomTrailer;

impl RandomTrailer for YamlRandomTrailer {
    fn prepare_string_with_tag(&self, input: &mut String, tag: &str) {
        write!(input, "{}", TagSuffix { tag }).expect("writing to a string always succeeds")
    }

    fn prepare_vec_with_tag(&self, input: &mut Vec<u8>, tag: &str) {
        write!(input, "{}", TagSuffix { tag }).expect("writing to a vec always succeeds")
    }

    fn remove_trailer(&self, string_like: &mut impl StringLike, tag: &str) -> bool {
        if string_like.ends_with_string(tag) {
            let target_len = string_like.len()
            - tag.len()
            // Both `"t"` and `"\t"` are 1 byte
            - 1;

            string_like.truncate_to_bytes(target_len);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
#[path = ""]
mod test {

    #[path = "../../tests/output/yaml_output/cow_string.rs"]
    mod cow_string;

    use cow_string::CowString;
    use serde::Deserialize;

    use super::*;

    const TAG: &str = "BERLIN";

    fn parse_and_undo_tag(prepared: &str) -> (CowString<'_>, bool) {
        let mut parsed: CowString =
            CowString::deserialize(serde_yaml::Deserializer::from_str(prepared)).unwrap();

        let had_tag = match &mut parsed {
            CowString::VisitBorrowedStr(s) => YamlRandomTrailer.remove_trailer(s, TAG),
            CowString::VisitStr { cloned } => YamlRandomTrailer.remove_trailer(cloned, TAG),
            CowString::VisitString(s) => YamlRandomTrailer.remove_trailer(s, TAG),
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
                r#"'Hello, wo"#,
                true,
                CowString::VisitBorrowedStr("Hello, wo"),
            ),
            (
                r#""Hello, wo\"#,
                true,
                // Unfinished backslash escape in YAML -> we can't know what it'll eventually represent.
                //
                // serde_yaml has to convert "\t" into <TAB>, so the parsed YAML is no longer a substring
                // of the YAML itself -> CowString::Owned.
                CowString::VisitStr {
                    cloned: "Hello, wo".to_string(),
                },
            ),
            (
                r#"'Hello, wo\"#,
                true,
                // Unfinished backslash escape in YAML -> we can't know what it'll eventually represent.
                //
                // serde_yaml has to convert "\t" into <TAB>, so the parsed YAML is no longer a substring
                // of the YAML itself -> CowString::Owned.
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
                r#"'Hello, world'"#,
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
            (
                // \n converts into newline
                r#"'Hello,\nworld'"#,
                false,
                CowString::VisitStr {
                    cloned: "Hello,\nworld".to_string(),
                },
            ),
        ] {
            let mut prepared = input.to_string();
            YamlRandomTrailer.prepare_string_with_tag(&mut prepared, TAG);
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
