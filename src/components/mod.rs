//! ## Components
//!
//! demo example components

use super::{Direction, Msg};

use tuirealm::props::{Alignment, Borders, Color, Style};

use tuirealm::tui::widgets::Block;

// -- modules
mod list;

// -- export
pub use list::List;

/// ### get_block
///
/// Get block
pub(crate) fn get_block<'a>(props: Borders, title: (String, Alignment), focus: bool) -> Block<'a> {
    Block::default()
        .borders(props.sides)
        .border_style(match focus {
            true => props.style(),
            false => Style::default().fg(Color::Reset).bg(Color::Reset),
        })
        .border_type(props.modifiers)
        .title(title.0)
        .title_alignment(title.1)
}
