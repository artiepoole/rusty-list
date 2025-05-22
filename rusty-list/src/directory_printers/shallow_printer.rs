use std::fs;
use std::path::PathBuf;

pub fn print_directory(path: &PathBuf) {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}
