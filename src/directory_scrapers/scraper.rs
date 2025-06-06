use std::path::PathBuf;
use std::{fs, io};

use crate::FsEntry;

pub fn scrape_fs(path: PathBuf, current_depth: usize, max_depth: usize) -> io::Result<FsEntry> {
    match fs::read_dir(&path) {
        Ok(paths) => {
            let mut children = Vec::new();
            if current_depth < max_depth {
                for entry in paths {
                    children.push(scrape_fs(entry?.path(), current_depth + 1, max_depth)?);
                }
            }
            Ok(FsEntry::Dir {
                path: path.clone(),
                children,
                depth: current_depth,
            })
        }
        Err(err)
        if matches!(err.kind(), io::ErrorKind::NotADirectory) => {
            Ok(FsEntry::File { path, depth: current_depth })
        }
        Err(err) => Err(err) // TODO: handle permission errors
    }
}
