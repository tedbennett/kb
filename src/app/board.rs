use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::{fs, io::Result};
use tui::widgets::TableState;

#[derive(Serialize, Deserialize, PartialEq)]
enum Mode {
    Normal,
    Move,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

#[derive(Serialize, Deserialize)]
pub struct Board<'a> {
    pub title: &'a str,
    pub columns: Vec<Column<'a>>,
    #[serde(default, skip_serializing)]
    pub selected_column: usize,
    #[serde(default, skip_serializing)]
    mode: Mode,
    #[serde(default, skip_serializing)]
    file_name: &'a str,
}

impl<'a> Board<'a> {
    pub fn on_keypress(&mut self, key: &KeyEvent) {
        let should_move = self.is_moving() || key.modifiers == KeyModifiers::SHIFT;
        match key.code {
            KeyCode::Down => self.down(should_move),
            KeyCode::Up => self.up(should_move),
            KeyCode::Left => self.left(should_move),
            KeyCode::Right => self.right(should_move),
            KeyCode::Esc => self.normal_mode(),
            KeyCode::Char('m') => self.toggle_mode(),
            _ => {}
        }
    }

    pub fn is_moving(&'a self) -> bool {
        self.mode == Mode::Move
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Normal => {
                if self.columns[self.selected_column]
                    .state
                    .selected()
                    .is_none()
                {
                    Mode::Normal
                } else {
                    Mode::Move
                }
            }
            Mode::Move => Mode::Normal,
        };
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn select_column(&mut self, index: usize) {
        if self.columns.len() == 0 {
            return;
        }
        self.selected_column = index;

        self.columns.iter_mut().enumerate().for_each(|(i, col)| {
            if i == index && col.rows.len() > 0 {
                col.state.select(Some(0));
            } else {
                col.state.select(None)
            }
        })
    }

    pub fn right(&mut self, move_row: bool) {
        let index = if self.selected_column >= self.columns.len() - 1 {
            0
        } else {
            self.selected_column + 1
        };
        if move_row {
            let Some(row) = self.columns[self.selected_column].state.selected() else { return };
            self.move_row(
                (self.selected_column, row),
                (index, self.columns[index].rows.len()),
            );
        } else {
            self.select_column(index);
        }
    }

    pub fn left(&mut self, move_row: bool) {
        let index = if self.selected_column == 0 {
            self.columns.len() - 1
        } else {
            self.selected_column - 1
        };

        if move_row {
            let Some(row) = self.columns[self.selected_column].state.selected() else { return };
            self.move_row(
                (self.selected_column, row),
                (index, self.columns[index].rows.len()),
            );
        } else {
            self.select_column(index);
        }
    }

    pub fn down(&mut self, move_row: bool) {
        assert!(self.selected_column < self.columns.len());
        let col = &mut self.columns[self.selected_column];
        if col.rows.len() == 0 {
            return;
        }
        let (origin, dest) = match col.state.selected() {
            Some(i) => {
                if i >= col.rows.len() - 1 {
                    (i, 0)
                } else {
                    (i, i + 1)
                }
            }
            None => (0, 0),
        };
        if move_row {
            self.move_row((self.selected_column, origin), (self.selected_column, dest));
        } else {
            col.state.select(Some(dest));
        }
    }

    pub fn up(&mut self, move_row: bool) {
        assert!(self.selected_column < self.columns.len());
        let col = &mut self.columns[self.selected_column];
        if col.rows.len() == 0 {
            return;
        }
        let (origin, dest) = match col.state.selected() {
            Some(i) => {
                if i == 0 {
                    (i, col.rows.len() - 1)
                } else {
                    (i, i - 1)
                }
            }
            None => (0, 0),
        };
        if move_row {
            self.move_row((self.selected_column, origin), (self.selected_column, dest))
        } else {
            col.state.select(Some(dest));
        }
    }

    pub fn move_row(&mut self, origin: (usize, usize), destination: (usize, usize)) {
        let popped = self.columns[origin.0].rows.remove(origin.1);
        self.columns[destination.0]
            .rows
            .insert(destination.1, popped);

        self.selected_column = destination.0;

        self.columns.iter_mut().enumerate().for_each(|(i, col)| {
            if i == destination.0 && col.rows.len() > 0 {
                col.state.select(Some(destination.1));
            } else {
                col.state.select(None)
            }
        });
        _ = self.save().expect("Failed to write to file");
    }

    pub fn from_file(file: &'a str, file_name: &'a str) -> Result<Self> {
        let mut board: Board = serde_json::from_str(file)?;
        board.select_column(0);
        board.file_name = file_name;
        Ok(board)
    }

    fn save(&self) -> Result<()> {
        let serialized = serde_json::to_string(self)?;
        fs::write(self.file_name, serialized)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TableState")]
struct TableStateDef {
    #[serde(getter = "TableState::selected")]
    selected: Option<usize>,
}

impl From<TableStateDef> for TableState {
    fn from(_def: TableStateDef) -> TableState {
        TableState::default()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Column<'a> {
    pub title: &'a str,
    pub rows: Vec<Row<'a>>,
    #[serde(default, skip_serializing, with = "TableStateDef")]
    pub state: TableState,
}

#[derive(Serialize, Deserialize)]
pub struct Row<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
