use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::fs;
use tui::widgets::TableState;

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub title: String,
    pub columns: Vec<Column>,
    #[serde(default, skip_serializing)]
    pub selected_column: usize,
    #[serde(default, skip_serializing)]
    filename: String,
}

impl Board {
    pub fn create(filename: &str) -> color_eyre::Result<Self> {
        _ = std::fs::File::create(filename)?;
        let board = Self::new("", filename);
        _ = board.save()?;
        Ok(board)
    }

    pub fn new(title: &str, filename: &str) -> Self {
        Self {
            title: title.to_string(),
            columns: Vec::new(),
            selected_column: 0,
            filename: filename.to_string(),
        }
    }
    pub fn selected_column(&mut self) -> Option<&mut Column> {
        if self.selected_column >= self.columns.len() {
            return None;
        }
        Some(&mut self.columns[self.selected_column])
    }
    pub fn selected_row(&mut self) -> Option<&mut Row> {
        let Some(col) = self.selected_column() else { return None;};
        let Some(index) = col.state.selected() else { return None; };
        Some(&mut col.rows[index])
    }

    pub fn on_keypress(&mut self, key: &KeyEvent) {
        let should_move = key.modifiers == KeyModifiers::SHIFT;
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => self.down(should_move),
            KeyCode::Up | KeyCode::Char('k') => self.up(should_move),
            KeyCode::Left | KeyCode::Char('h') => self.left(should_move),
            KeyCode::Right | KeyCode::Char('l') => self.right(should_move),
            _ => {}
        }
    }

    pub fn insert_row(&mut self, title: String, description: String) {
        let Some(col) = self.selected_column() else { return };
        col.rows.push(Row { title, description });
        _ = self.save();
    }

    pub fn update_row(&mut self, title: String, description: String) {
        let Some(row) = self.selected_row() else { return };
        *row = Row { title, description };
        _ = self.save()
    }

    pub fn delete_row(&mut self) {
        let Some(col) = self.selected_column() else { return };
        let Some(index) = col.state.selected() else { return };

        let new_selection: Option<usize> = {
            if col.rows.is_empty() {
                None
            } else {
                if index == 0 {
                    Some(0)
                } else {
                    Some(index - 1)
                }
            }
        };
        col.state.select(new_selection);
        _ = col.rows.remove(index);
        _ = self.save();
    }

    pub fn create_column(&mut self, title: String) {
        self.columns.push(Column {
            title,
            rows: Vec::new(),
            state: TableState::default(),
        });
        _ = self.save();
    }

    pub fn update_column(&mut self, title: String) {
        let Some(col) = self.selected_column() else { return};
        col.title = title;
        _ = self.save();
    }

    pub fn delete_column(&mut self) {
        if self.selected_column >= self.columns.len() {
            return;
        }
        self.columns.remove(self.selected_column);
        self.select_column(0);
        _ = self.save();
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
        let Some(col) = self.selected_column() else { return };
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
        let Some(col) = self.selected_column() else { return };
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

    pub fn from_file(file: String, file_name: String) -> color_eyre::Result<Self> {
        let mut board: Board = serde_json::from_str(&file)?;
        board.select_column(0);
        board.filename = file_name;
        Ok(board)
    }

    fn save(&self) -> color_eyre::Result<()> {
        let serialized = serde_json::to_string(self)?;
        fs::write(&self.filename, serialized)?;
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
pub struct Column {
    pub title: String,
    pub rows: Vec<Row>,
    #[serde(default, skip_serializing, with = "TableStateDef")]
    pub state: TableState,
}

#[derive(Serialize, Deserialize)]
pub struct Row {
    pub title: String,
    pub description: String,
}
