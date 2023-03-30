use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::TextArea;

use super::board::Board;

#[derive(PartialEq)]
enum Mode {
    Create,
    Edit,
    Normal,
    Delete,
}

#[derive(PartialEq)]
pub enum PopupFocusState {
    Description,
    Title,
}

impl Default for PopupFocusState {
    fn default() -> Self {
        Self::Title
    }
}

impl PopupFocusState {
    pub fn title(&self) -> &str {
        match self {
            Self::Description => "Description",
            Self::Title => "Title",
        }
    }

    pub fn placeholder(&self) -> &str {
        match self {
            Self::Description => "Description\nPress CTRL-D to Submit",
            Self::Title => "Title",
        }
    }
}

impl<'a> PopupState<'a> {
    fn cycle_focus(&mut self) {
        self.focussed = match self.focussed {
            PopupFocusState::Title => PopupFocusState::Description,
            PopupFocusState::Description => PopupFocusState::Title,
        }
    }
}

#[derive(Default)]
pub struct PopupState<'a> {
    pub title: TextArea<'a>,
    pub description: TextArea<'a>,
    pub focussed: PopupFocusState,
}

#[derive(Default, PartialEq)]
pub enum DialogFocusState {
    #[default]
    Confirm,
    Cancel,
}

impl DialogFocusState {
    pub fn text(&self) -> &str {
        match self {
            Self::Confirm => " Confirm ",
            Self::Cancel => " Cancel ",
        }
    }
}

#[derive(Default)]
pub struct DialogState {
    pub focussed: DialogFocusState,
    pub message: String,
}

impl DialogState {
    pub fn cycle_focus(&mut self) {
        self.focussed = match self.focussed {
            DialogFocusState::Confirm => DialogFocusState::Cancel,
            DialogFocusState::Cancel => DialogFocusState::Confirm,
        }
    }
}

pub struct Model<'a> {
    pub board: Board<'a>,
    pub popup: PopupState<'a>,
    pub dialog: DialogState,
    mode: Mode,
    pub quit: bool,
}

impl<'a> Model<'a> {
    pub fn new(board: Board<'a>) -> Self {
        Model {
            board,
            popup: PopupState::default(),
            dialog: DialogState::default(),
            mode: Mode::Normal,
            quit: false,
        }
    }

    pub fn is_creating(&self) -> bool {
        self.mode == Mode::Create
    }

    pub fn is_editing(&self) -> bool {
        self.mode == Mode::Edit
    }

    pub fn is_deleting(&self) -> bool {
        self.mode == Mode::Delete
    }

    pub fn hide_popup(&mut self) {
        self.mode = Mode::Normal;
        self.popup = PopupState::default();
    }

    pub fn hide_dialog(&mut self) {
        self.mode = Mode::Normal;
        self.dialog = DialogState::default();
    }

    pub fn edit_item(&mut self) {
        let Some(row) = self.board.selected_row() else { return };
        self.popup = PopupState {
            title: TextArea::new(row.title.split('\n').map(|s| s.to_string()).collect()),
            description: TextArea::new(
                row.description.split('\n').map(|s| s.to_string()).collect(),
            ),
            focussed: PopupFocusState::Title,
        };
        self.mode = Mode::Edit;
    }

    fn open_delete_dialog(&mut self) {
        if self.board.selected_row().is_none() {
            return;
        }
        self.mode = Mode::Delete;
    }

    pub fn create_item(&mut self) {
        self.board.insert_row(
            self.popup.title.lines().join("\n"),
            self.popup.description.lines().join("\n"),
        );
        self.hide_popup();
    }

    pub fn update_item(&mut self) {
        self.board.update_row(
            self.popup.title.lines().join("\n"),
            self.popup.description.lines().join("\n"),
        );
        self.hide_popup();
    }

    pub fn delete_item(&mut self) {
        self.board.delete_row();
        self.hide_dialog();
    }

    pub fn on_keypress(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
            self.quit = true;
            return;
        }
        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char('c') => self.mode = Mode::Create,
                KeyCode::Enter | KeyCode::Char('e') => self.edit_item(),
                KeyCode::Backspace | KeyCode::Char('d') => self.open_delete_dialog(),
                _ => self.board.on_keypress(&key),
            },
            Mode::Create => match key {
                KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => self.create_item(),
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => self.hide_popup(),
                _ => self.popup.on_keypress(key),
            },
            Mode::Edit => match key {
                KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => self.update_item(),
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => self.hide_popup(),
                _ => self.popup.on_keypress(key),
            },
            Mode::Delete => match key.code {
                KeyCode::Esc => self.hide_dialog(),
                KeyCode::Enter => {
                    if self.dialog.focussed == DialogFocusState::Confirm {
                        self.delete_item();
                    }
                    self.hide_dialog();
                }
                _ => self.dialog.on_keypress(key),
            },
        }
    }
}

impl<'a> PopupState<'a> {
    fn on_keypress(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Tab {
            self.cycle_focus();
            return;
        }
        if self.focussed == PopupFocusState::Title && key.code == KeyCode::Enter {
            self.focussed = PopupFocusState::Description;
            return;
        }
        _ = match self.focussed {
            PopupFocusState::Title => self.title.input(key),
            PopupFocusState::Description => self.description.input(key),
        }
    }
}

impl DialogState {
    fn on_keypress(&mut self, key: KeyEvent) {
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
