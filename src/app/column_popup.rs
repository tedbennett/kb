use crossterm::event::KeyEvent;
use tui_textarea::{CursorMove, TextArea};

use super::PopupFields;

#[derive(PartialEq, Default)]
pub enum ColumnFields {
    #[default]
    Title,
}

pub struct ColumnPopupState<'a> {
    pub title: TextArea<'a>,
}

impl PopupFields for ColumnFields {
    fn title(&self) -> &str {
        match self {
            Self::Title => "Title",
        }
    }

    fn placeholder(&self) -> &str {
        match self {
            Self::Title => "Enter title...",
        }
    }
}

impl<'a> ColumnPopupState<'a> {
    pub fn new(title: &str) -> Self {
        let mut new = Self {
            title: title.lines().map(|s| s.to_string()).collect(),
        };
        new.title.move_cursor(CursorMove::End);
        new
    }
    pub fn on_keypress(&mut self, key: KeyEvent) {
        self.title.input(key);
    }
}
