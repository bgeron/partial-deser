use std::borrow::Cow;

#[path = "cow_string.rs"]
mod cow_string;

use cow_string::CowString;

#[test]
fn test_borrowed_string_advanced_api() {
    let partial_yaml = r#"["abc", "de\nf", "unterminated"#;

    let options = partial_deser::Options::new_yaml();
    let prepared = options.prepare_str_for_borrowed_deserialization(Cow::Borrowed(partial_yaml));

    let value: Vec<CowString> = options.from_yaml_str_borrowed(&prepared).unwrap();

    insta::assert_ron_snapshot!(value, @r###"
    [
      VisitBorrowedStr("abc"),
      VisitStr(
        cloned: "de\nf",
      ),
      VisitStr(
        cloned: "unterminated",
      ),
    ]
    "###);
}
