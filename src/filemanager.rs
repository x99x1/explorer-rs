use std::io;
use std::path::{Path, PathBuf};

use crate::filesystem::{FileEntry, list_directory};

pub struct FileManager {
    pub entries: Vec<FileEntry>,
    pub selected: usize,
    pub current_path: PathBuf
}

impl FileManager {
    pub fn new(path: &Path) -> io::Result<Self> {
        let absolute_path: PathBuf = path.canonicalize()?;
        Ok(Self {
            entries: Self::sort(&absolute_path),
            selected: 0,
            current_path: absolute_path 
        })
    }

    pub fn next(&mut self) {
        if !self.entries.is_empty() {
            self.selected = (self.selected + 1) % self.entries.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.entries.is_empty() {
            self.selected = self.selected.saturating_sub(1);
        }
    }

    pub fn refresh(&mut self, path: &Path) -> io::Result<()> {
        self.current_path = path.canonicalize()?;
        self.entries = Self::sort(&self.current_path);
        self.selected = 0;
        Ok(())
    }

    pub fn get_selected(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected)
    }

    fn sort(path: &Path) -> Vec<FileEntry> {
        let mut entries: Vec<FileEntry> = list_directory(path).unwrap_or_default();
        entries.sort_by( | a: &FileEntry, b: &FileEntry | {
            if a.is_dir && !b.is_dir { std::cmp::Ordering::Less
            } else if !a.is_dir && b.is_dir { std::cmp::Ordering::Greater
            } else { a.name.cmp(&b.name) }
        });

        entries
    }
}