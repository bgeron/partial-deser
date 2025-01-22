use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget as _, Wrap};
use ratatui::TerminalOptions;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use tokio_util::bytes;
use tokio_util::io::ReaderStream;
use tracing::error;
use tui_textarea::TextArea;

use crate::generic::display::ActiveDisplay;
use crate::generic::{self};
use crate::Args;

mod input;

#[derive(Debug)]
enum Event {
    Quit,
    /// A character came either from terminal, or non-terminal stdin
    TextArea(crossterm::event::KeyEvent),
    Recompute,
    Error(anyhow::Error),
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let display = generic::display::Display::init(&args.display).await;

    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(input::handle_terminal_input(tx.clone()));
    tokio::spawn(input::handle_nonterminal_input(tx.clone()));

    main_loop(args, display, tx, rx).await;

    Ok(())
}

async fn main_loop(
    args: Args,
    mut display: Box<dyn ActiveDisplay>,
    tx: mpsc::UnboundedSender<Event>,
    mut rx: mpsc::UnboundedReceiver<Event>,
) {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: ratatui::Viewport::Inline(16),
    });

    let mut textarea = TextArea::default();

    let mut state = String::new();

    tx.send(Event::Recompute).unwrap();

    while let Some(event) = rx.recv().await {
        match event {
            Event::TextArea(event) => {
                textarea.input(event);
                tx.send(Event::Recompute).unwrap();
            }
            Event::Recompute => {
                let contents = textarea.lines().join("\n");
                let result = args.schema.parse(&args.format, contents.as_bytes());
                state = display.display(result).await;
            }

            Event::Quit => {
                break;
            }
            Event::Error(error) => error!("{error}"),
        }

        terminal
            .draw(|f| {
                let layout = Layout::vertical(Constraint::from_fills([1, 2])).vertical_margin(1);
                let [textarea_area, result_area] = layout.areas(f.area());
                f.render_widget(&textarea, textarea_area);

                f.render_widget(
                    Paragraph::new(&*state)
                        .block(Block::bordered().title("Output"))
                        // .gray()
                        .wrap(Wrap { trim: false }),
                    result_area,
                );
            })
            .expect("could not render");
    }
}
