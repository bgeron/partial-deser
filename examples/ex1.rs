#![allow(unused_imports)]

use indexmap::IndexMap;
use partial_deser::unstable::UnstableCustomBehavior;
use tracing::level_filters::LevelFilter;

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(LevelFilter::TRACE)
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .init();

    {
        let json = "true";
        let parsed: bool = partial_deser::from_json_str(json).unwrap();
        dbg!(parsed);
    }

    {
        let json = "[true] x";
        let parsed: Vec<bool> = partial_deser::from_json_str(json).unwrap();
        dbg!(parsed);
    }

    {
        let yaml = r#"[["abc", "def"], ["abc", "de\
        
# '"#;
        let parsed: Vec<Vec<String>> = partial_deser::from_yaml_str(yaml).unwrap();
        dbg!(parsed);
    }

    {
        let yaml = "{\"abc\":";
        let parsed: IndexMap<String, String> = partial_deser::from_yaml_str(yaml).unwrap();
        dbg!(parsed);
    }

    if false {
        let yaml = r#"
- ["abc", "def"]
- ["abc", "de
        
# '"#;
        let parsed: Vec<Vec<String>> = partial_deser::from_yaml_str(yaml).unwrap();
        dbg!(parsed);
    }

    if false {
        let yaml = dbg!(
            r#"[["abc", "def"]]


#'"#
        );
        let parsed: Vec<Vec<String>> = partial_deser::from_yaml_str(yaml).unwrap();
        dbg!(parsed);
    }

    // {
    //     let json = "[";

    //     let parsed: Vec<bool> = partial_deser::Options::new_json()
    //         .from_json_str(json.into())
    //         .unwrap();
    //     dbg!(parsed);
    // }

    // {
    //     let json = "[";
    //     let mut behavior = UnstableCustomBehavior::default();
    //     behavior.unstable_fallback_seq_empty = false;
    //     behavior.unstable_fallback_seq_skip_item = false;

    //     let parsed: Vec<bool> = partial_deser::Options::new_json()
    //         .custom_behavior(behavior)
    //         .from_json_str(json.into())
    //         .unwrap();
    //     dbg!(parsed);
    // }

    // {
    //     let json = "[true, false, tru";
    //     let parsed: Vec<bool> = partial_deser::Options::new_json()
    //         .custom_behavior(UnstableCustomBehavior::default().no_fallbacks())
    //         .with_max_n_backtracks(Some(1))
    //         .from_json_str(json.into())
    //         .unwrap();
    //     dbg!(parsed);
    // }

    // {
    //     let json = "[true, false, 3, 4.5, \"hello";
    //     let parsed: serde_json::Value = partial_deser::Options::new_json()
    //         .from_json_str(json.into())
    //         .unwrap();
    //     dbg!(parsed);
    // }

    // {
    //     let json = r#"[{"ab""#;
    //     let parsed: Vec<indexmap::IndexMap<String, String>> = partial_deser::Options::new_json()
    //         .from_json_str(json.into())
    //         .unwrap();
    //     dbg!(parsed);
    // }
}
