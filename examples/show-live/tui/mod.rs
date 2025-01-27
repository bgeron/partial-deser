use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyModifiers};
use futures::{FutureExt, Stream};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget as _, Wrap};
use ratatui::TerminalOptions;
use tokio::io::AsyncReadExt;
use tokio::pin;
use tokio::sync::{mpsc, watch};
use tokio::time::sleep;
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream, WatchStream};
use tokio_stream::{once, StreamExt as _};
use tokio_util::bytes;
use tokio_util::io::ReaderStream;
use tracing::error;
use tui_textarea::TextArea;

use crate::shared::display::{ActiveDisplay, DisplayPreference};
use crate::shared::format::ParseSettings;
use crate::Args;

mod input;
mod process;

#[derive(Debug)]
enum Event {
    EndOfInput,
    /// A character came either from terminal, or non-terminal stdin
    TextArea(crossterm::event::KeyEvent),
    RecomputedDisplay(Text<'static>),
    AfterLastDisplay,
    GoFullscreen,
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let display = DisplayPreference::init(&args.output).await;

    let (tx, rx) = mpsc::unbounded_channel();
    let (input_tx, input_rx) = watch::channel::<Option<String>>(None);
    let displayed_rx = process::compute_displays_from_inputs(args.clone(), input_rx, display);

    tokio::spawn(input::handle_terminal_input(tx.clone()));
    tokio::spawn(input::handle_nonterminal_input(tx.clone()));

    let rx = futures::stream_select!(
        UnboundedReceiverStream::new(rx),
        WatchStream::new(displayed_rx)
            .map(Event::RecomputedDisplay)
            .chain(once(Event::AfterLastDisplay)),
    );

    main_loop(args, input_tx, rx).await;

    ratatui::restore();

    Ok(())
}

async fn main_loop(
    args: Args,
    input_tx: watch::Sender<Option<String>>,
    rx: impl Stream<Item = Event>,
) {
    let mut input_tx = Some(input_tx); // so we can close the input
    let terminal_options = if args.tui_height > 0 {
        TerminalOptions {
            viewport: ratatui::Viewport::Inline(args.tui_height),
        }
    } else {
        TerminalOptions {
            viewport: ratatui::Viewport::Fullscreen,
        }
    };
    let mut terminal = ratatui::init_with_options(terminal_options);
    terminal.clear().expect("could not clear");

    let mut textarea = TextArea::default();

    let mut displayed = Text::default();

    pin!(rx);
    while let Some(mut event) = rx.next().await {
        let mut last_iteration = false;
        loop {
            match event {
                Event::TextArea(event) => {
                    textarea.input(event);
                    if let Some(input_tx) = &input_tx {
                        let _old_value = input_tx.send_replace(Some(textarea.lines().join("\n")));
                    }
                }
                Event::RecomputedDisplay(text) => {
                    displayed = text;
                }
                Event::GoFullscreen => {
                    terminal = ratatui::init_with_options(TerminalOptions {
                        viewport: ratatui::Viewport::Fullscreen,
                    });
                    terminal.clear().expect("could not clear");
                    terminal.autoresize().expect("could not autoresize");
                }

                Event::EndOfInput => {
                    // Close input.
                    input_tx = None;
                    tokio::spawn(async {
                        sleep(Duration::from_secs(3)).await;
                        error!("Quit has been requested but program still has not exited");
                    });
                }

                Event::AfterLastDisplay => {
                    last_iteration = true;
                }
            }
            if let Some(additional_event) = rx.next().now_or_never() {
                event = additional_event.unwrap();
            } else {
                break;
            }
        }

        terminal
            .draw(|f| {
                let textarea_lines = textarea.lines().len();

                let layout = Layout::vertical([
                    Constraint::Length(u16::try_from(textarea_lines + 2).ok().unwrap_or(u16::MAX)),
                    Constraint::Fill(1),
                ])
                .vertical_margin(1);
                let [textarea_area, result_area] = layout.areas(f.area());
                f.render_widget(&textarea, textarea_area);

                f.render_widget(
                    Paragraph::new(displayed.clone())
                        .block(Block::bordered().title("Output"))
                        .wrap(Wrap { trim: false }),
                    result_area,
                );
            })
            .expect("could not render");

        if last_iteration {
            break;
        }
    }
}
