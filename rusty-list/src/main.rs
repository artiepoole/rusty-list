use clap::Parser;
use std::path::PathBuf;

use env_logger;

mod printers;
use crate::printers::{print_directory, print_recursive};

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
    #[arg(short = 'R', long)]
    recursive: bool,
    /// limit the depth for recursion
    #[arg(short, long, default_value = "0")]
    depth: usize,
}

fn main() {
    env_logger::init();

    let args = Cli::parse();

    println!("long mode: {:?}", args.long_mode);
    println!("all mode: {:?}", args.all);
    println!("file order: {:?}", args.directory_order);
    println!("recursive mode: {:?}", args.recursive);
    println!("max depth: {}", args.depth);
    let path = args.path.unwrap();
    if !path.exists() {
        panic!("path '{}' doesn't exist", path.display());
    }
    if args.recursive {
        print_recursive(&path, args.depth)
    } else {
        print_directory(&path)
    }
}
