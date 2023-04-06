#![allow(dead_code)]
use crossterm::event::{KeyCode, KeyEvent};
use tui::widgets::TableState;

pub struct FilePickerState {
    pub files: Vec<String>,
    pub state: TableState,
}

impl FilePickerState {

    pub fn new(dir: &str) -> Self {
        let files: Vec<String> = match std::fs::read_dir(dir) {
            Ok(dir) => dir
                .filter_map(|f| f.ok())
                .filter_map(|f| f.file_name().into_string().ok())
                .filter(|f| f.ends_with(".json"))
                .collect(),
            Err(_) => Vec::new(),
        };
        Self {
            files,
            state: TableState::default(),
        }    
    }
    
    pub fn on_keypress(&mut self, key: KeyEvent) {
        let Some(selected) = self.state.selected() else {
            if !self.files.is_empty() {
                self.state.select(Some(0));
            }
            return; 
        };
        match key.code {
            KeyCode::Up => self.state.select(Some(
                if selected == 0 {
                    self.files.len() - 1
                } else {
                    selected - 1
                })
            ),
            KeyCode::Down => self.state.select(Some(
                if selected == self.files.len() - 1 {
                     0
                } else { 
                    selected + 1
                })
             ),
            _ => {}
        }
    }
}
