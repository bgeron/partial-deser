use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    let json = "true";

    let parsed: bool = partial_deser::from_json_str(&json).unwrap();

    dbg!(parsed);
}
