use serde::{Deserialize, Serialize};
use std::io::Result;
use tui::widgets::TableState;

#[derive(Serialize, Deserialize)]
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
    selected_column: usize,
    #[serde(default, skip_serializing)]
    selected_row: usize,
    #[serde(default, skip_serializing)]
    mode: Mode,
}

impl<'a> Board<'a> {
    pub fn selected_column(&'a self) -> &'a Column {
        assert!(self.selected_column < self.columns.len());
        &self.columns[self.selected_column]
    }

    pub fn current_state(&'a mut self) -> &'a mut TableState {
        assert!(self.selected_column < self.columns.len());
        &mut self.columns[self.selected_column].state
    }

    pub fn right(&mut self) {
        if self.selected_column >= self.columns.len() - 1 {
            return;
        }
        self.selected_column += 1;
        self.columns.iter_mut().enumerate().for_each(|(i, col)| {
            if i == self.selected_column {
                col.state.select(Some(0));
            } else {
                col.state.select(None)
            }
        })
    }

    pub fn left(&mut self) {
        if self.selected_column == 0 {
            return;
        }
        self.selected_column -= 1;

        self.columns.iter_mut().enumerate().for_each(|(i, col)| {
            if i == self.selected_column {
                col.state.select(Some(0));
            } else {
                col.state.select(None)
            }
        })
    }

    pub fn down(&mut self) {
        assert!(self.selected_column < self.columns.len());
        let col = &mut self.columns[self.selected_column];
        let i = match col.state.selected() {
            Some(i) => {
                if i >= col.rows.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        col.state.select(Some(i));
    }

    pub fn up(&mut self) {
        assert!(self.selected_column < self.columns.len());
        let col = &mut self.columns[self.selected_column];
        let i = match col.state.selected() {
            Some(i) => {
                if i == 0 {
                    col.rows.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        col.state.select(Some(i));
    }

    pub fn from_file(file: &'a str) -> Result<Self> {
        let board = serde_json::from_str(file)?;
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
