use crossterm::event::{KeyCode, KeyEvent};

use super::PopupFields;

#[derive(Default, PartialEq)]
pub enum DialogFields {
    #[default]
    Confirm,
    Cancel,
}

impl PopupFields for DialogFields {
    fn title(&self) -> &str {
        match self {
            Self::Confirm => " Confirm ",
            Self::Cancel => " Cancel ",
        }
    }
    fn placeholder(&self) -> &str {
        ""
    }
}

#[derive(Default)]
pub struct DialogState {
    pub focussed: DialogFields,
    pub message: String,
}

impl DialogState {
    pub fn cycle_focus(&mut self) {
        self.focussed = match self.focussed {
            DialogFields::Confirm => DialogFields::Cancel,
            DialogFields::Cancel => DialogFields::Confirm,
        }
    }
}

impl DialogState {
    pub fn on_keypress(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                code: KeyCode::Tab, ..
            }
            | KeyEvent {
                code: KeyCode::Left,
                ..
            }
            | KeyEvent {
                code: KeyCode::Right,
                ..
            } => self.cycle_focus(),
            _ => {}
        }
    }
}
