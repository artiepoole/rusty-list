use std::fs;
use std::fs::metadata;
use std::path::PathBuf;
use log::debug;

#[derive(Debug)]
pub struct RecursivePrinter {
    pub max_depth: usize,
    pub current_depth: usize,
}

impl RecursivePrinter {
    pub fn print_directory_recursive(&mut self, path: &PathBuf) {
        self.current_depth += 1;
        debug!("Recursive depth increased to: {}", self.current_depth);

        let paths = fs::read_dir(path).unwrap();
        for dir_entry in paths {
            let path = dir_entry.unwrap().path();
            if metadata(&path).unwrap().is_dir() && (self.max_depth == 0 || self.current_depth < self.max_depth) {
                self.print_directory_recursive(&path);
            } else {
                println!("{}", path.display());
            }
        }
        self.current_depth -= 1;
        debug!("Recursive depth decreased to: {}", self.current_depth);
    }
}
