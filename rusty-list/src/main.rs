use std::{env, fs};
use std::path::PathBuf;



fn main() {
    let mut args: Vec<String> = env::args().collect();
    // remove first argument which is self
    args.remove(0);
    let path = PathBuf::from(args.pop().unwrap_or_else(|| "./".to_owned()));
    if !path.exists(){panic!("path doesn't exist")}
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }

    // println!("path: {:?}", path.display());

}
