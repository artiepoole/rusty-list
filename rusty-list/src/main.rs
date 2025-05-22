use std::env;
use std::path::PathBuf;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // remove first argument which is self
    args.remove(0);
    let pathstr = args.pop().unwrap_or_else(|| "./".to_owned());


    let path = PathBuf::from(pathstr);
    
    println!("path: {:?}", path.display());

}
