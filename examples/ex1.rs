// use tracing::info;

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
}
