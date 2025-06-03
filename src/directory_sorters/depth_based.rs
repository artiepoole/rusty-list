use std::path::PathBuf;

pub fn depth_only_sort(all_paths: &mut Vec<PathBuf>) {
    all_paths.sort_by_key(|path| path.components().count());
}

pub fn depth_alpha_sort(all_paths: &mut Vec<PathBuf>) {
    // Sorts by depth if different, else by lexicographic.
    all_paths.sort_by(|a, b| {
        let depth_a = a.components().count();
        let depth_b = b.components().count();

        // chatgpt used: 
        // match depth_a.cmp(&depth_b) {
        //     std::cmp::Ordering::Equal => a.cmp(b), // alphabetical fallback
        //     other => other,
        // }
        
        // I find this more intuitive. 
        if depth_a == depth_b {
            a.cmp(b) // alphabetical fallback
        } else {
            depth_a.cmp(&depth_b)
        }
    });
}
