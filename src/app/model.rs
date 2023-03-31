use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::TextArea;

use super::{
    board::Board,
    dialog::DialogState,
    row_popup::{RowFields, RowPopupState},
    ColumnPopupState, DialogFields,
};

#[derive(PartialEq)]
enum Mode {
    CreateRow,
    EditRow,
    Normal,
    DeleteRow,
    CreateColumn,
    EditColumn,
    DeleteColumn,
}

pub struct Model<'a> {
    pub board: Board<'a>,
    pub popup: RowPopupState<'a>,
    pub dialog: DialogState,
    pub column: ColumnPopupState<'a>,
    mode: Mode,
    pub quit: bool,
}

impl<'a> Model<'a> {
    pub fn new(board: Board<'a>) -> Self {
        Model {
            board,
            popup: RowPopupState::default(),
            dialog: DialogState::default(),
            column: ColumnPopupState::default(),
            mode: Mode::Normal,
            quit: false,
        }
    }

    pub fn is_creating(&self) -> bool {
        self.mode == Mode::CreateRow
    }

    pub fn is_editing(&self) -> bool {
        self.mode == Mode::EditRow
    }

    pub fn is_deleting(&self) -> bool {
        self.mode == Mode::DeleteRow
    }

    pub fn is_creating_column(&self) -> bool {
        self.mode == Mode::CreateColumn
    }

    pub fn is_editing_column(&self) -> bool {
        self.mode == Mode::EditColumn
    }

    pub fn is_deleting_column(&self) -> bool {
        self.mode == Mode::DeleteColumn
    }

    pub fn hide_popup(&mut self) {
        self.mode = Mode::Normal;
        self.popup = RowPopupState::default();
    }

    pub fn hide_dialog(&mut self) {
        self.mode = Mode::Normal;
        self.dialog = DialogState::default();
    }

    pub fn hide_column_popup(&mut self) {
        self.mode = Mode::Normal;
        self.column = ColumnPopupState::default();
    }
    pub fn edit_item(&mut self) {
        let Some(row) = self.board.selected_row() else { return };
        self.popup = RowPopupState {
            title: TextArea::new(row.title.split('\n').map(|s| s.to_string()).collect()),
            description: TextArea::new(
                row.description.split('\n').map(|s| s.to_string()).collect(),
            ),
            focussed: RowFields::Title,
        };
        self.mode = Mode::EditRow;
    }

    pub fn edit_column(&mut self) {
        let Some(col) = self.board.selected_column() else { return };
        self.column = ColumnPopupState {
            title: TextArea::new(col.title.split('\n').map(|s| s.to_string()).collect()),
            focussed: super::ColumnFields::Title,
        };
        self.mode = Mode::EditColumn;
    }

    fn open_delete_dialog(&mut self) {
        if self.board.selected_row().is_none() {
            return;
        }
        self.mode = Mode::DeleteRow;
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

    pub fn create_column(&mut self) {
        self.board.create_column(self.column.title.lines().join(""));
        self.hide_column_popup();
    }
    pub fn update_column(&mut self) {
        self.board.update_column(self.column.title.lines().join(""));
        self.hide_column_popup();
    }
    pub fn delete_column(&mut self) {
        self.board.delete_column();
        self.hide_dialog();
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
                KeyCode::Char('c') => self.mode = Mode::CreateRow,
                KeyCode::Char('C') => self.mode = Mode::CreateColumn,
                KeyCode::Char('E') => self.edit_column(),
                KeyCode::Enter | KeyCode::Char('e') => self.edit_item(),
                KeyCode::Backspace | KeyCode::Char('d') => self.open_delete_dialog(),
                KeyCode::Char('D') => self.mode = Mode::DeleteColumn,
                _ => self.board.on_keypress(&key),
            },
            Mode::CreateRow => match key {
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
            Mode::EditRow => match key {
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
            Mode::DeleteRow => match key.code {
                KeyCode::Esc => self.hide_dialog(),
                KeyCode::Enter => {
                    if self.dialog.focussed == DialogFields::Confirm {
                        self.delete_item();
                    }
                    self.hide_dialog();
                }
                _ => self.dialog.on_keypress(key),
            },
            Mode::CreateColumn => match key.code {
                KeyCode::Esc => self.hide_column_popup(),
                KeyCode::Enter => self.create_column(),
                _ => self.column.on_keypress(key),
            },
            Mode::EditColumn => match key.code {
                KeyCode::Esc => self.hide_column_popup(),
                KeyCode::Enter => self.update_column(),
                _ => self.column.on_keypress(key),
            },
            Mode::DeleteColumn => match key.code {
                KeyCode::Esc => self.hide_dialog(),
                KeyCode::Enter => {
                    if self.dialog.focussed == DialogFields::Confirm {
                        self.delete_column();
                    }
                    self.hide_dialog();
                }
                _ => self.dialog.on_keypress(key),
            },
        }
    }
}
