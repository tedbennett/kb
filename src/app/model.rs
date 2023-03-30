use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::TextArea;

use super::board::Board;

#[derive(PartialEq)]
enum Mode {
    Create,
    Edit,
    Move,
    Normal,
}

enum Direction {
    Down,
    Left,
    Right,
    Up,
}

enum Msg {
    ChangeMode(Mode),
    MoveCursor(Direction),
    MoveRow(Direction),
    Submit,
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

pub struct Model<'a> {
    pub board: Board<'a>,
    pub popup: PopupState<'a>,
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

    pub fn show_popup(&self) -> bool {
        self.mode == Mode::Create
    }

    pub fn hide_popup(&mut self) {
        self.mode = Mode::Normal;
        self.popup = PopupState::default();
    }

    pub fn create_item(&mut self) {
        self.board.insert_row(
            self.popup.title.lines().join("\n"),
            self.popup.description.lines().join("\n"),
        );
        self.hide_popup();
    }

    pub fn on_keypress(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('q')
            || (key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL)
        {
            self.quit = true;
            return;
        }
        match self.mode {
            Mode::Move | Mode::Normal => {
                if key.code == KeyCode::Char('c') {
                    self.mode = Mode::Create;
                    return;
                }
                self.board.on_keypress(&key)
            }
            Mode::Create => {
                if key.code == KeyCode::Char('d') && key.modifiers == KeyModifiers::CONTROL {
                    self.create_item();
                    return;
                }
                if key.code == KeyCode::Esc {
                    self.hide_popup();
                    return;
                }
                self.popup.on_keypress(key);
            }
            Mode::Edit => self.popup.on_keypress(key),
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
