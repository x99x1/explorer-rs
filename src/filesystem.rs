use std::io;
use std::path::Path;
use walkdir::{WalkDir, DirEntry};

pub fn list_directory(path: &Path) -> io::Result<Vec<String>> {
    let mut dirs: Vec<String> = Vec::with_capacity(128);

    for entry in WalkDir::new(path).max_depth(1).min_depth(1) {
        let entry: DirEntry = entry?;
        let entry_path: String = entry.file_name().to_string_lossy().to_string();
        dirs.push(entry_path);
    }

    return Ok(dirs);
}