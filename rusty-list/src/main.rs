use clap::Parser;
use std::path::PathBuf;
use env_logger;
use log::debug;
mod directory_printers;


use directory_printers::{
    recursive_printer::{RecursivePrinter},
    shallow_printer::{print_directory}
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // optional positional
    /// The starting directory for search
    #[arg(default_value = "./")]
    path: PathBuf,
    
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
    debug!("search root: {:?}", args.path);
    debug!("long mode: {:?}", args.long_mode);
    debug!("all mode: {:?}", args.all);
    debug!("file order: {:?}", args.directory_order);
    debug!("recursive mode: {:?}", args.recursive);
    debug!("max depth: {}", args.depth);
    let path = args.path;
    if !path.exists() {
        panic!("path '{}' doesn't exist", path.display());
    }
    if args.recursive {
        let mut path_printer = RecursivePrinter{
            max_depth: args.depth,
            current_depth: 0
        };
        path_printer.print_directory_recursive(&path);
    } else {
        print_directory(&path)
    }
}
