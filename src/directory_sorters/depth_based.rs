use std::path::PathBuf;

pub fn depth_only_sort(all_paths: &mut Vec<PathBuf>) {
    all_paths.sort_by_key(|path| path.components().count());
}



pub fn depth_alpha_sort(all_paths: &mut Vec<PathBuf>) {


    // Sorts by depth if different, else by lexicographic.
    all_paths.sort_by(|a, b| {
        let a_num_components = a.components().count();
        let b_num_components = b.components().count();
        a_num_components.cmp(&b_num_components).reverse()
            .then_with(|| a.cmp(&b))

    });
}
