use log::debug;
use once_cell::sync::Lazy;
use std::fs;
use std::fs::metadata;
use std::path::PathBuf;
use std::sync::Mutex;
static RECURSION_DEPTH: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub fn print_recursive(path: &PathBuf, max_depth: usize) {
    let depth_counter = {
        let mut depth = RECURSION_DEPTH.lock().unwrap();
        *depth += 1;
        *depth
    };

    debug!("Recursive depth increased to: {}", depth_counter);

    let paths = fs::read_dir(path).unwrap();
    for dir_entry in paths {
        let path = dir_entry.unwrap().path();
        if metadata(&path).unwrap().is_dir() && (max_depth == 0 || depth_counter < max_depth) {
            print_recursive(&path, max_depth);
        } else {
            println!("{}", path.display());
        }
    }
    let depth_counter = {
        let mut depth = RECURSION_DEPTH.lock().unwrap();
        *depth -= 1;
        *depth
    };
    debug!("Recursive depth decreased to: {}", depth_counter);
}

pub fn print_directory(path: &PathBuf) {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}
