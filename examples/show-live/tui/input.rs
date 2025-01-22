use std::io::IsTerminal;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyEvent, KeyModifiers};
use ratatui::TerminalOptions;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use tokio_util::bytes::{self, BytesMut};
use tokio_util::io::ReaderStream;
use tracing::{error, warn};

use crate::generic::display::ActiveDisplay;
use crate::generic::{self};
use crate::Args;

use super::Event;

pub(super) async fn handle_terminal_input(tx: mpsc::UnboundedSender<Event>) {
    while let Some(event) = EventStream::new().next().await {
        match event {
            Ok(CrosstermEvent::Key(event)) if is_quit_key(event) => {
                tx.send(Event::Quit).unwrap();
                continue;
            }

            Ok(CrosstermEvent::Key(event)) => tx.send(Event::TextArea(event)).unwrap(),

            Ok(event) => warn!(?event, "Unrecognized event"),

            Err(error) => {
                error!(?error, "Error reading terminal inputs");
                break;
            }
        }
    }
}

fn is_quit_key(event: crossterm::event::KeyEvent) -> bool {
    [KeyCode::Char('c'), KeyCode::Char('d'), KeyCode::Char('z')].contains(&event.code)
        && event.modifiers.contains(KeyModifiers::CONTROL)
        || event.code == KeyCode::Esc
}

pub(super) async fn handle_nonterminal_input(tx: mpsc::UnboundedSender<Event>) {
    if std::io::stdin().is_terminal() {
        // Input is handled by another function
        return;
    }

    let mut buf = Vec::with_capacity(8);

    loop {
        let bytes_read = tokio::io::stdin().read_buf(&mut buf).await.unwrap();
        if bytes_read == 0 {
            break;
        }

        while let Some(c) = pop_codepoint_front(&mut buf) {
            tx.send(Event::TextArea(KeyCode::Char(c).into())).unwrap();
        }
    }

    tx.send(Event::Quit).unwrap();
}

fn pop_codepoint_front(buf: &mut Vec<u8>) -> Option<char> {
    // Characters in UTF-8 are at most 4 bytes.
    for end_index in 1..=4.min(buf.len()) {
        if let Ok(s) = std::str::from_utf8(&buf[..end_index]) {
            let c = s.chars().next().unwrap();
            // Remove this character from the front. Linear time in size
            // of the buffer, so use a small buffer.
            buf.drain(..end_index);
            return Some(c);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pop_codepoint_front() {
        /// The emoji is a family of two men and two boys
        let mut buf = "¡Señores 👨‍👨‍👦‍👦 hi!".as_bytes().to_vec();
        assert_eq!(pop_codepoint_front(&mut buf), Some('¡'));
        assert_eq!(pop_codepoint_front(&mut buf), Some('S'));
        assert_eq!(pop_codepoint_front(&mut buf), Some('e'));
        assert_eq!(pop_codepoint_front(&mut buf), Some('ñ'));
        assert_eq!(pop_codepoint_front(&mut buf), Some('o'));
        assert_eq!(pop_codepoint_front(&mut buf), Some('r'));
        assert_eq!(pop_codepoint_front(&mut buf), Some('e'));
        assert_eq!(pop_codepoint_front(&mut buf), Some('s'));
        assert_eq!(pop_codepoint_front(&mut buf), Some(' '));
        assert_eq!(pop_codepoint_front(&mut buf), Some('\u{1f468}')); // man
        assert_eq!(pop_codepoint_front(&mut buf), Some('\u{200d}')); // zwj
        assert_eq!(pop_codepoint_front(&mut buf), Some('\u{1f468}')); // man
        assert_eq!(pop_codepoint_front(&mut buf), Some('\u{200d}')); // zwj
        assert_eq!(pop_codepoint_front(&mut buf), Some('\u{1f466}')); // boy
        assert_eq!(pop_codepoint_front(&mut buf), Some('\u{200d}')); // zwj
        assert_eq!(pop_codepoint_front(&mut buf), Some('\u{1f466}')); // boy
        assert_eq!(buf, b" hi!");
        assert_eq!(pop_codepoint_front(&mut buf), Some(' '));
        assert_eq!(pop_codepoint_front(&mut buf), Some('h'));
        assert_eq!(pop_codepoint_front(&mut buf), Some('i'));
        assert_eq!(pop_codepoint_front(&mut buf), Some('!'));
        assert_eq!(pop_codepoint_front(&mut buf), None);
    }
}
