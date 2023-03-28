use serde::{Deserialize, Serialize};
use std::io::Result;
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
}

impl<'a> Board<'a> {
    pub fn is_moving(&'a self) -> bool {
        self.mode == Mode::Move
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Normal => Mode::Move,
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

    pub fn right(&mut self) {
        let index = if self.selected_column >= self.columns.len() - 1 {
            0
        } else {
            self.selected_column + 1
        };
        match self.mode {
            Mode::Normal => self.select_column(index),
            Mode::Move => {
                let Some(row) = self.columns[self.selected_column].state.selected() else { return };
                self.move_row(
                    (self.selected_column, row),
                    (index, self.columns[index].rows.len()),
                )
            }
        };
    }

    pub fn left(&mut self) {
        let index = if self.selected_column == 0 {
            self.columns.len() - 1
        } else {
            self.selected_column - 1
        };

        match self.mode {
            Mode::Normal => self.select_column(index),
            Mode::Move => {
                let Some(row) = self.columns[self.selected_column].state.selected() else { return };
                self.move_row(
                    (self.selected_column, row),
                    (index, self.columns[index].rows.len()),
                )
            }
        }
    }

    pub fn down(&mut self) {
        assert!(self.selected_column < self.columns.len());
        let col = &mut self.columns[self.selected_column];
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
        match self.mode {
            Mode::Normal => col.state.select(Some(dest)),
            Mode::Move => {
                self.move_row((self.selected_column, origin), (self.selected_column, dest))
            }
        }
    }

    pub fn up(&mut self) {
        assert!(self.selected_column < self.columns.len());
        let col = &mut self.columns[self.selected_column];
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
        match self.mode {
            Mode::Move => {
                self.move_row((self.selected_column, origin), (self.selected_column, dest))
            }
            Mode::Normal => col.state.select(Some(dest)),
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
        })
    }
    // pub fn move_in_col(&mut self, destination: usize) {

    // }

    pub fn from_file(file: &'a str) -> Result<Self> {
        let mut board: Board = serde_json::from_str(file)?;
        board.select_column(0);
        Ok(board)
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
