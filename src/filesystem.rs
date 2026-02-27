use std::io;
use std::path::Path;
use std::fs::{DirEntry, ReadDir, read_dir};
use std::ffi::OsString;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: OsString,
    pub is_dir: bool
}

pub fn list_directory(path: &Path) -> io::Result<Vec<FileEntry>> {
    let iter: ReadDir = read_dir(path)?;
    let mut entries: Vec<FileEntry> = Vec::with_capacity(iter.size_hint().0);
    for entry in iter {
        let e: DirEntry = entry?;
        entries.push(FileEntry { name: e.file_name(), is_dir: e.file_type()?.is_dir() });
    }
    Ok(entries)
}