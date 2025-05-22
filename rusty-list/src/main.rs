use std::{fs};
use std::fs::metadata;
use std::ops::Deref;
use std::path::PathBuf;
use clap::{Parser};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // optional positional
    /// The starting directory for search
    #[arg(default_value = "./")]
    path: Option<PathBuf>,

    // optional flags
    /// Use a long listing format
    #[arg(short)]
    long_mode: bool,
    /// Do not ignore entries starting with .
    #[arg(short, long)]
    all: bool,
    /// List all entries in directory order
    #[arg(short = 'f')]
    directory_order: bool,
    /// List subdirectories recursively
    #[arg(short='R', long)]
    recursive: bool,
    /// limit the depth for recursion
    #[arg(short, long, default_value="0")]
    depth: usize,

}

static RECURSION_DEPTH: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

fn print_recursive(path: &PathBuf, max_depth: usize) {
    let depth_counter = {
        let mut depth = RECURSION_DEPTH.lock().unwrap();
        *depth += 1;
        *depth
    };
    
    println!("Recursive depth increased to: {}", depth_counter);

    let paths = fs::read_dir(path).unwrap();
    for dir_entry in paths {
        let path = dir_entry.unwrap().path();
        if (metadata(&path).unwrap().is_dir() && (max_depth == 0 || depth_counter < max_depth)) {
            print_recursive(&path, max_depth);
        }
        else {
            println!("{}", path.display());
        }
    }
    let depth_counter = {
        let mut depth = RECURSION_DEPTH.lock().unwrap();
        *depth -= 1;
        *depth
    };
    println!("Recursive depth decreased to: {}", depth_counter);
}

fn print_directory(path: &PathBuf) {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}
fn main() {
    let args = Cli::parse();

    println!("long mode: {:?}", args.long_mode);
    println!("all mode: {:?}", args.all);
    println!("file order: {:?}", args.directory_order);
    println!("recursive mode: {:?}", args.recursive);
    println!("max depth: {}", args.depth);
    let path = args.path.unwrap();
    if !path.exists(){panic!("path '{}' doesn't exist", path.display());}
    if (args.recursive){print_recursive(&path, args.depth)} else {print_directory(&path)}


}
