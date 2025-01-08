use serde::{Deserialize, Serialize};

use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_unit_variant() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    enum Enum {
        UnitVariant,
    };

    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Enum>>(&default_modes(), br#"["UnitVariant", "UnitVariant"]"#),
        @"");
}
