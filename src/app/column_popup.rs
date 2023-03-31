use crossterm::event::KeyEvent;
use tui_textarea::TextArea;

use super::PopupFields;

#[derive(PartialEq, Default)]
pub enum ColumnFields {
    #[default]
    Title,
    Confirm,
}

#[derive(Default)]
pub struct ColumnPopupState<'a> {
    pub focussed: ColumnFields,
    pub title: TextArea<'a>,
}

impl PopupFields for ColumnFields {
    fn title(&self) -> &str {
        match self {
            Self::Title => "Title",
            Self::Confirm => "Confirm",
        }
    }

    fn placeholder(&self) -> &str {
        match self {
            Self::Title => "Enter title...",
            Self::Confirm => "",
        }
    }
}

impl<'a> ColumnPopupState<'a> {
    pub fn on_keypress(&mut self, key: KeyEvent) {
        self.title.input(key);
    }
}
