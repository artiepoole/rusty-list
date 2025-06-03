use std::path::PathBuf;

pub fn tree_like_printer(all_paths: &Vec<PathBuf>) {
    println!("WARN  tree like printing is WIP. Resorting to a simple print.");
    for path in all_paths {
        println!("{:?}", path);
    }
}
