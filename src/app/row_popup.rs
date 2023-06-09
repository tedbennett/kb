use crossterm::event::{KeyCode, KeyEvent};
use tui_textarea::{CursorMove, TextArea};

pub trait PopupFields {
    fn title(&self) -> &str;
    fn placeholder(&self) -> &str;
}

#[derive(PartialEq, Default)]
pub enum RowFields {
    Description,
    #[default]
    Title,
}

impl PopupFields for RowFields {
    fn title(&self) -> &str {
        match self {
            Self::Description => "Description",
            Self::Title => "Title",
        }
    }

    fn placeholder(&self) -> &str {
        match self {
            Self::Description => "Description\nPress CTRL-D to Submit",
            Self::Title => "Title",
        }
    }
}

impl<'a> RowPopupState<'a> {
    fn cycle_focus(&mut self) {
        self.focussed = match self.focussed {
            RowFields::Title => RowFields::Description,
            RowFields::Description => RowFields::Title,
        }
    }
}

#[derive(Default)]
pub struct RowPopupState<'a> {
    pub title: TextArea<'a>,
    pub description: TextArea<'a>,
    pub focussed: RowFields,
}

impl<'a> RowPopupState<'a> {
    pub fn new(title: &str, description: &str) -> Self {
        let mut new = Self {
            title: TextArea::new(title.lines().map(|s| s.to_string()).collect()),
            description: TextArea::new(description.lines().map(|s| s.to_string()).collect()),
            focussed: RowFields::Title,
        };
        new.title.move_cursor(CursorMove::End);
        new.description.move_cursor(CursorMove::End);
        new
    }

    pub fn on_keypress(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Tab {
            self.cycle_focus();
            return;
        }
        if self.focussed == RowFields::Title && key.code == KeyCode::Enter {
            self.focussed = RowFields::Description;
            return;
        }
        _ = match self.focussed {
            RowFields::Title => self.title.input(key),
            RowFields::Description => self.description.input(key),
        }
    }
}
