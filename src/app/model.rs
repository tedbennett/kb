use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{
    board::Board, dialog::DialogState, row_popup::RowPopupState, ColumnPopupState, DialogFields,
};

pub enum Popup<'a> {
    None,
    CreateRow(RowPopupState<'a>),
    EditRow(RowPopupState<'a>),
    DeleteRow(DialogState),
    CreateColumn(ColumnPopupState<'a>),
    EditColumn(ColumnPopupState<'a>),
    DeleteColumn(DialogState),
}

pub struct Model<'a> {
    pub board: Board<'a>,
    pub popup: Popup<'a>,
    pub quit: bool,
}

impl<'a> Model<'a> {
    pub fn new(board: Board<'a>) -> Self {
        let empty_board = board.columns.is_empty();
        Model {
            board,
            popup: if empty_board {
                Popup::CreateColumn(ColumnPopupState::new(""))
            } else {
                Popup::None
            },
            quit: false,
        }
    }

    pub fn edit_item(&mut self) {
        let Some(row) = self.board.selected_row() else { return };
        self.popup = Popup::EditRow(RowPopupState::new(&row.title, &row.description));
    }

    pub fn edit_column(&mut self) {
        let Some(col) = self.board.selected_column() else { return };
        self.popup = Popup::EditColumn(ColumnPopupState::new(&col.title));
    }

    fn open_delete_dialog(&mut self) {
        if self.board.selected_row().is_none() {
            return;
        }
        self.popup = Popup::DeleteRow(DialogState::new("Delete Item?"));
    }

    pub fn create_item(&mut self, title: &str, description: &str) {
        self.board
            .insert_row(title.to_string(), description.to_string());
        self.popup = Popup::None;
    }

    pub fn update_item(&mut self, title: &str, description: &str) {
        self.board
            .update_row(title.to_string(), description.to_string());
        self.popup = Popup::None;
    }

    pub fn create_column(&mut self, title: &str) {
        self.board.create_column(title.to_string());
        self.popup = Popup::None;
    }
    pub fn update_column(&mut self, title: &str) {
        self.board.update_column(title.to_string());
        self.popup = Popup::None;
    }
    pub fn delete_column(&mut self) {
        self.board.delete_column();
        self.popup = if self.board.columns.is_empty() {
            Popup::CreateColumn(ColumnPopupState::new(""))
        } else {
            Popup::None
        };
    }

    pub fn delete_item(&mut self) {
        self.board.delete_row();
        self.popup = Popup::None;
    }

    pub fn on_keypress(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
            self.quit = true;
            return;
        }
        match &mut self.popup {
            Popup::None => match key.code {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char('c') => self.popup = Popup::CreateRow(RowPopupState::default()),
                KeyCode::Char('C') => self.popup = Popup::CreateColumn(ColumnPopupState::new("")),
                KeyCode::Char('E') => self.edit_column(),
                KeyCode::Enter | KeyCode::Char('e') => self.edit_item(),
                KeyCode::Backspace | KeyCode::Char('d') => self.open_delete_dialog(),
                KeyCode::Char('D') => {
                    self.popup = Popup::DeleteColumn(DialogState::new("Delete Column?"))
                }
                _ => self.board.on_keypress(&key),
            },
            Popup::CreateRow(state) => match key {
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
                } => self.popup = Popup::None,
                _ => state.on_keypress(key),
            },
            Popup::EditRow(state) => match key {
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
                } => self.popup = Popup::None,
                _ => state.on_keypress(key),
            },
            Popup::DeleteRow(state) => match key.code {
                KeyCode::Esc => self.popup = Popup::None,
                KeyCode::Enter => {
                    if state.focussed == DialogFields::Confirm {
                        self.delete_item();
                    }
                    self.popup = Popup::None;
                }
                _ => state.on_keypress(key),
            },
            Popup::CreateColumn(state) => match key.code {
                KeyCode::Esc => self.popup = Popup::None,
                KeyCode::Enter => {
                    let title = &state.title.lines().join("");
                    self.create_column(title);
                }
                _ => state.on_keypress(key),
            },
            Popup::EditColumn(state) => match key.code {
                KeyCode::Esc => self.popup = Popup::None,
                KeyCode::Enter => {
                    let title = &state.title.lines().join("");
                    self.update_column(title);
                }
                _ => state.on_keypress(key),
            },
            Popup::DeleteColumn(state) => match key.code {
                KeyCode::Esc => self.popup = Popup::None,
                KeyCode::Enter => {
                    if state.focussed == DialogFields::Confirm {
                        self.delete_column();
                    } else {
                        self.popup = Popup::None;
                    }
                }
                _ => state.on_keypress(key),
            },
        }
    }
}
