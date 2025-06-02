use log::debug;
use std::{fs, io};
use std::path::PathBuf;

#[derive(Debug)]
pub struct RecursivePrinter {
    pub max_depth: usize,
    pub current_depth: usize,
}

impl RecursivePrinter {
    pub fn print_directory_recursive(&mut self, path: &PathBuf) -> io::Result<()> {
        self.current_depth += 1;
        debug!("Recursive depth increased to: {}", self.current_depth);

        let paths = fs::read_dir(path)?;
        for dir_entry in paths {
            let path = dir_entry?;
            if path.path().is_dir() && (self.max_depth == 0 || self.current_depth < self.max_depth) 
            {
                self.print_directory_recursive(&path.path()).expect(&format!("Failed to recursively print {:?}", path));
            } else {
                println!("{:?}", path.path());
            }
        }
        self.current_depth -= 1;
        debug!("Recursive depth decreased to: {}", self.current_depth);
        Ok(())
    }
}
