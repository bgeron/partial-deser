use crate::common::{
    run_json_on_prefixes_and_format_outputs, run_on_prefixes_and_format_outputs,
    stringify_output_errors,
};

#[test]
fn test_bools() {
    assert_eq!(
        run_json_on_prefixes_and_format_outputs::<Vec<bool>>(b"[true, false, true]"),
        vec![]
    );
}
