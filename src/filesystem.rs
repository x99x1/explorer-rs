use std::io;
use std::path::Path;
use std::fs::{DirEntry, read_dir};
use std::ffi::OsString;

pub fn list_directory(path: &Path) -> io::Result<Vec<OsString>> {
    read_dir(path)?.map( | entry: Result<DirEntry, io::Error> | {
        Ok(entry?.file_name())}).collect()
}