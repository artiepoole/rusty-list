use std::path::PathBuf;

pub fn sort_purely_alphabetically(all_paths: &mut Vec<PathBuf>) {
    all_paths.sort_by(|a, b| a.cmp(b));
}
