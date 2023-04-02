use crossterm::event::KeyEvent;
use tui_textarea::TextArea;

use super::PopupFields;

#[derive(PartialEq, Default)]
pub enum ColumnFields {
    #[default]
    Title,
}

#[derive(Default)]
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
    pub fn on_keypress(&mut self, key: KeyEvent) {
        self.title.input(key);
    }
}
