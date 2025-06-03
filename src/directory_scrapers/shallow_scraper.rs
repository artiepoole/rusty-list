use std::path::PathBuf;
use std::{fs, io};

pub fn scrape_directory(path: &PathBuf, all_paths: & mut Vec::<PathBuf>) -> io::Result<()> {
    let paths = fs::read_dir(path)?;
    for path in paths {
        let path = path?;
        all_paths.push(path.path());
    }
    Ok(())
}
