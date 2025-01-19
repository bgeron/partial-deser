#![allow(unused_imports)]

use std::io::Read;

use clap::{Parser, ValueEnum};
use generic::display::Display;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

#[path = "parse/generic/mod.rs"]
mod generic;

use generic::format::FormatAndSettings;
use generic::schema::Schema;

/// Parse input JSON incrementally as it comes in, and show the results
/// live in the terminal.
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    display: Option<Display>,

    #[clap(long, default_value_t)]
    #[arg(value_enum)]
    format: FormatAndSettings,

    #[clap(long, default_value_t)]
    #[arg(value_enum)]
    schema: Schema,

    /// Whether to add a random trailer to the input before parsing.
    ///
    /// For JSON, this enables incremental strings. For YAML, this
    /// does too, and is really important to prevent flickering.
    ///
    /// Random trailers are format-specific.
    #[clap(long, default_value_t = true)]
    use_random_trailer: bool,
}

fn main() {
    tracing_subscriber::fmt::fmt()
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .init();

    let args = Args::parse();
    dbg!(&args);

    let display = generic::display::Display::init(&args.display);

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let result = args.schema.parse(args.format, input.as_bytes());
    display.display(&result);

    dbg!(result).unwrap();
}
