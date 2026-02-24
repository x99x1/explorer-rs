use std::path::{Path, PathBuf};

use crate::filesystem::list_directory;

pub struct FileManager {
    pub items: Vec<String>,
    pub selected: usize,
    pub current_path: String
}

impl FileManager {
    pub fn new(path: &Path) -> Self {
        let absolute_path: PathBuf = path.canonicalize().unwrap_or_else(| _ | path.to_path_buf());

        Self {
            items: list_directory(&absolute_path).unwrap(),
            selected: 0,
            current_path: absolute_path.display().to_string()
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1) % self.items.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            if self.selected == 0 {
                self.selected = self.items.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    pub fn refresh(&mut self, path: &Path) {
        let absolute_path: PathBuf = path.canonicalize().unwrap_or_else(| _ | path.to_path_buf());

        self.items = list_directory(&absolute_path).unwrap();
        self.selected = 0;
        self.current_path = absolute_path.display().to_string();
    }

    pub fn get_selected(&self) -> Option<&String> {
        self.items.get(self.selected)
    }
}