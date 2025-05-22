use std::path::PathBuf;
use std::{fs, io};

pub fn print_directory(path: &PathBuf) -> io::Result<()> {
    let paths = fs::read_dir(path)?;
    for path in paths {
        let path = path?;
        println!("{:?}", path.path());
    }
    Ok(())
}
