use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::board::Board;

enum Mode {
    Normal,
    Move,
    Create,
    Edit,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

enum Msg {
    MoveCursor(Direction),
    MoveRow(Direction),
    ChangeMode(Mode),
    Submit,
}

#[derive(Default)]
struct PopupState {
    title: String,
    description: String,
}

pub struct Model<'a> {
    pub board: Board<'a>,
    popup: PopupState,
    mode: Mode,
    pub quit: bool,
}

impl<'a> Model<'a> {
    pub fn new(board: Board<'a>) -> Self {
        Model {
            board,
            popup: PopupState::default(),
            mode: Mode::Normal,
            quit: false,
        }
    }
    pub fn on_keypress(&mut self, key: &KeyEvent) {
        if key.code == KeyCode::Char('q')
            || (key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL)
        {
            self.quit = true;
            return;
        }
        match self.mode {
            Mode::Move | Mode::Normal => self.board.on_keypress(key),
            Mode::Create | Mode::Edit => self.popup.on_keypress(key),
        }
    }
}

impl PopupState {
    fn on_keypress(&mut self, key: &KeyEvent) {}
}
