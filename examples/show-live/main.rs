#![allow(unused_imports)]

use std::io::Read;

use clap::{Parser, ValueEnum};
use generic::display::DisplayPreference;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

#[path = "../parse/generic/mod.rs"]
mod generic;
mod tui;

use generic::format::FormatAndSettings;
use generic::schema::Schema;

/// Parse input JSON incrementally as it comes in, and show the results
/// live in the terminal.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about=None)]
struct Args {
    /// How to display the parsed data. Default: use nushell when installed.
    #[clap(short, long)]
    display: Vec<DisplayPreference>,

    #[clap(short, long, default_value_t)]
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .init();

    let args = Args::parse();

    tui::main(args).await
}
