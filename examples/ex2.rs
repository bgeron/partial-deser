use serde::{Deserialize, Serialize};

use tracing::level_filters::LevelFilter;

fn assert_eq_as_ron(left: &impl Serialize, right_ron_str: &str) {
    let left_ron = ron::from_str::<ron::Value>(dbg!(&ron::to_string(left).unwrap())).unwrap();
    let right_ron = ron::from_str(right_ron_str).expect("right string was not valid RON");
    assert_eq!(left_ron, right_ron);
}

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
        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        struct Newtype(Vec<()>);

        dbg!(serde_json::to_string_pretty(&Newtype(vec![(), ()])).unwrap());
    }

    {
        #[derive(Debug, Default, Deserialize, Serialize)]
        struct TravelMode {
            #[serde(default)]
            mode: String,
            benefit: Option<String>,
        }

        let json = r#"[{"mode": "foot", "benefit": "healthy"}, {"mode": "aeropl"#;
        let modes: Vec<TravelMode> = deser_incomplete::from_json_str(json).unwrap();

        assert_eq_as_ron(
            &modes,
            r#"[
               TravelMode { mode: "foot", benefit: Some("healthy") },
               TravelMode { mode: "aeropl", benefit: None },
            ]"#,
        );
    }
}
