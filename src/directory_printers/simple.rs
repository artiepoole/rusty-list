use std::path::PathBuf;

pub fn simple_printer(all_paths: &Vec<PathBuf>) {
    for path in all_paths {
        println!("{:?}", path);
    }
}
