use std::borrow::Cow;

#[path = "cow_string.rs"]
mod cow_string;

use cow_string::CowString;

#[test]
fn test_borrowed_string_advanced_api() {
    let partial_json = r#"["abc", "de\nf", "unterminated"#;

    let options = partial_deser::Options::new_json();
    let prepared = options.prepare_str_for_borrowed_deserialization(Cow::Borrowed(partial_json));

    let value: Vec<CowString> = options.from_json_str_borrowed_unstable(&prepared).unwrap();

    insta::assert_ron_snapshot!(value, @r###"
    [
      VisitBorrowedStr("abc"),
      VisitStr(
        cloned: "de\nf",
      ),
      VisitBorrowedStr("unterminated"),
    ]
    "###);
}
