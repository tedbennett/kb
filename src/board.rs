use serde::{Deserialize, Serialize};
use std::io::Result;

fn default_selection() -> usize {
    0
}
#[derive(Serialize, Deserialize)]
pub struct Board<'a> {
    pub title: &'a str,
    pub columns: Vec<Column<'a>>,
    #[serde(default = "default_selection")]
    selected_column: usize,
    #[serde(default = "default_selection")]
    selected_row: usize,
}

impl<'a> Board<'a> {
    pub fn new(title: &'a str, columns: Vec<Column<'a>>) -> Self {
        Self {
            title,
            columns,
            selected_column: 0,
            selected_row: 0,
        }
    }

    pub fn selected_column(&'a self) -> &'a Column {
        assert!(self.selected_column < self.columns.len());
        &self.columns[self.selected_column]
    }

    pub fn selected_row(&'a self) -> &'a Row {
        let column = self.selected_column();
        assert!(self.selected_row < column.rows.len());
        &column.rows[self.selected_row]
    }

    pub fn from_file(file: &'a str) -> Result<Self> {
        let board = serde_json::from_str(file)?;
        Ok(board)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Column<'a> {
    pub title: &'a str,
    pub rows: Vec<Row<'a>>,
}

#[derive(Serialize, Deserialize)]
pub struct Row<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
