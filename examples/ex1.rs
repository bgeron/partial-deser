// use tracing::info;

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
        let json = "[true, false, true, faaa";
        let parsed: Vec<bool> = partial_deser::from_json_str(json).unwrap();
        dbg!(parsed);
    }

    {
        let json = "[";

        let parsed: Vec<bool> = partial_deser::Options::new_json()
            .from_json_str(json)
            .unwrap();
        dbg!(parsed);
    }

    {
        let json = "[";
        let mut behavior = UnstableCustomBehavior::default();
        behavior.unstable_fallback_seq_empty = false;
        behavior.unstable_fallback_seq_skip_item = false;

        let parsed: Vec<bool> = partial_deser::Options::new_json()
            .custom_behavior(behavior)
            .from_json_str(json)
            .unwrap();
        dbg!(parsed);
    }

    {
        let json = "[true, false, tru";
        let parsed: Vec<bool> = partial_deser::Options::new_json()
            .custom_behavior(UnstableCustomBehavior::default().no_fallbacks())
            .with_max_n_backtracks(Some(1))
            .from_json_str(json)
            .unwrap();
        dbg!(parsed);
    }

    {
        let json = "[null]";
        let parsed: Vec<()> = partial_deser::Options::new_json()
            .custom_behavior(UnstableCustomBehavior::strict())
            .with_max_n_backtracks(Some(0))
            .from_json_str(json)
            .unwrap();
        dbg!(parsed);
    }
}
