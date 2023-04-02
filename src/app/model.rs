use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::TextArea;

use super::{
    board::Board,
    dialog::DialogState,
    row_popup::{RowFields, RowPopupState},
    ColumnPopupState, DialogFields,
};

// #[derive(PartialEq)]
pub enum Mode<'a> {
    CreateRow(RowPopupState<'a>),
    EditRow(RowPopupState<'a>),
    Normal,
    DeleteRow(DialogState),
    CreateColumn(ColumnPopupState<'a>),
    EditColumn(ColumnPopupState<'a>),
    DeleteColumn(DialogState),
}

pub struct Model<'a> {
    pub board: Board<'a>,
    pub mode: Mode<'a>,
    pub quit: bool,
}

impl<'a> Model<'a> {
    pub fn new(board: Board<'a>) -> Self {
        Model {
            board,
            mode: Mode::Normal,
            quit: false,
        }
    }

    pub fn edit_item(&mut self) {
        let Some(row) = self.board.selected_row() else { return };
        let state = RowPopupState {
            title: TextArea::new(row.title.split('\n').map(|s| s.to_string()).collect()),
            description: TextArea::new(
                row.description.split('\n').map(|s| s.to_string()).collect(),
            ),
            focussed: RowFields::Title,
        };
        self.mode = Mode::EditRow(state);
    }

    pub fn edit_column(&mut self) {
        let Some(col) = self.board.selected_column() else { return };
        let state = ColumnPopupState {
            title: TextArea::new(col.title.split('\n').map(|s| s.to_string()).collect()),
        };
        self.mode = Mode::EditColumn(state);
    }

    fn open_delete_dialog(&mut self) {
        if self.board.selected_row().is_none() {
            return;
        }
        self.mode = Mode::DeleteRow(DialogState::new("Delete Item?"));
    }

    pub fn create_item(&mut self, title: &str, description: &str) {
        self.board
            .insert_row(title.to_string(), description.to_string());
        self.mode = Mode::Normal;
    }

    pub fn update_item(&mut self, title: &str, description: &str) {
        self.board
            .update_row(title.to_string(), description.to_string());
        self.mode = Mode::Normal;
    }

    pub fn create_column(&mut self, title: &str) {
        self.board.create_column(title.to_string());
        self.mode = Mode::Normal;
    }
    pub fn update_column(&mut self, title: &str) {
        self.board.update_column(title.to_string());
        self.mode = Mode::Normal;
    }
    pub fn delete_column(&mut self) {
        self.board.delete_column();
        self.mode = Mode::Normal;
    }

    pub fn delete_item(&mut self) {
        self.board.delete_row();
        self.mode = Mode::Normal;
    }

    pub fn on_keypress(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
            self.quit = true;
            return;
        }
        match &mut self.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char('c') => self.mode = Mode::CreateRow(RowPopupState::default()),
                KeyCode::Char('C') => self.mode = Mode::CreateColumn(ColumnPopupState::default()),
                KeyCode::Char('E') => self.edit_column(),
                KeyCode::Enter | KeyCode::Char('e') => self.edit_item(),
                KeyCode::Backspace | KeyCode::Char('d') => self.open_delete_dialog(),
                KeyCode::Char('D') => {
                    self.mode = Mode::DeleteColumn(DialogState::new("Delete Column?"))
                }
                _ => self.board.on_keypress(&key),
            },
            Mode::CreateRow(state) => match key {
                KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    let title = &state.title.lines().join("");
                    let description = &state.description.lines().join("\n").clone();
                    self.create_item(title, description)
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => self.mode = Mode::Normal,
                _ => state.on_keypress(key),
            },
            Mode::EditRow(state) => match key {
                KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    let title = &state.title.lines().join("");
                    let description = &state.description.lines().join("\n");
                    self.update_item(title, description)
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => self.mode = Mode::Normal,
                _ => state.on_keypress(key),
            },
            Mode::DeleteRow(state) => match key.code {
                KeyCode::Esc => self.mode = Mode::Normal,
                KeyCode::Enter => {
                    if state.focussed == DialogFields::Confirm {
                        self.delete_item();
                    }
                    self.mode = Mode::Normal;
                }
                _ => state.on_keypress(key),
            },
            Mode::CreateColumn(state) => match key.code {
                KeyCode::Esc => self.mode = Mode::Normal,
                KeyCode::Enter => {
                    let title = &state.title.lines().join("");
                    self.create_column(title);
                }
                _ => state.on_keypress(key),
            },
            Mode::EditColumn(state) => match key.code {
                KeyCode::Esc => self.mode = Mode::Normal,
                KeyCode::Enter => {
                    let title = &state.title.lines().join("");
                    self.update_column(title);
                }
                _ => state.on_keypress(key),
            },
            Mode::DeleteColumn(state) => match key.code {
                KeyCode::Esc => self.mode = Mode::Normal,
                KeyCode::Enter => {
                    if state.focussed == DialogFields::Confirm {
                        self.delete_column();
                    }
                    self.mode = Mode::Normal;
                }
                _ => state.on_keypress(key),
            },
        }
    }
}
