use std::sync::Arc;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget as _, Wrap};
use ratatui::TerminalOptions;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use tokio_util::bytes;
use tokio_util::io::ReaderStream;
use tracing::error;
use tui_textarea::TextArea;

use crate::generic::display::{ActiveDisplay, DisplayPreference};
use crate::generic::format::ParseSettings;
use crate::generic::{self};
use crate::Args;

mod input;

#[derive(Debug)]
enum Event {
    Quit,
    /// A character came either from terminal, or non-terminal stdin
    TextArea(crossterm::event::KeyEvent),
    Recompute,
    GoFullscreen,
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let display = DisplayPreference::init(&args.output).await;

    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(input::handle_terminal_input(tx.clone()));
    tokio::spawn(input::handle_nonterminal_input(tx.clone()));

    main_loop(args, display, tx, rx).await;

    ratatui::restore();

    Ok(())
}

async fn main_loop(
    args: Args,
    mut display: Box<dyn ActiveDisplay>,
    tx: mpsc::UnboundedSender<Event>,
    mut rx: mpsc::UnboundedReceiver<Event>,
) {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: ratatui::Viewport::Inline(24),
    });

    let mut textarea = TextArea::default();

    let mut state = Text::default();

    tx.send(Event::Recompute).unwrap();

    while let Some(event) = rx.recv().await {
        match event {
            Event::TextArea(event) => {
                textarea.input(event);
                tx.send(Event::Recompute).unwrap();
            }
            Event::Recompute => {
                make_output_state(&args, &mut display, &textarea, &mut state).await;
            }
            Event::GoFullscreen => {
                terminal = ratatui::init_with_options(TerminalOptions {
                    viewport: ratatui::Viewport::Fullscreen,
                });
                terminal.clear().expect("could not clear");
                terminal.autoresize().expect("could not autoresize");
            }

            Event::Quit => break,
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
                    Paragraph::new(state.clone())
                        .block(Block::bordered().title("Output"))
                        // .gray()
                        .wrap(Wrap { trim: false }),
                    result_area,
                );
            })
            .expect("could not render");
    }
}

async fn make_output_state(
    args: &Args,
    display: &mut Box<dyn ActiveDisplay>,
    textarea: &TextArea<'_>,
    state: &mut Text<'_>,
) {
    let contents = textarea.lines().join("\n");
    let result = args.schema.parse(
        &args.format,
        &ParseSettings {
            use_random_trailer: args.use_random_trailer,
        },
        contents.as_bytes(),
    );
    let displayed = display.display(Arc::new(result)).await;

    *state = if contents.trim().is_empty() {
        Text::styled(
            "No input yet. Start typing to get a deserialization.\n\n",
            Style::default().gray().bold().italic(),
        )
    } else {
        Text::from(displayed)
    };
}
