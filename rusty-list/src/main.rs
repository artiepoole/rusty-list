use std::{fs};
use std::path::PathBuf;
use clap::{Parser};

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

}
fn main() {
    let args = Cli::parse();

    println!("long mode: {:?}", args.long_mode);
    println!("all mode: {:?}", args.all);
    println!("file order: {:?}", args.directory_order);
    println!("recursive mode: {:?}", args.recursive);

    let path = args.path.unwrap();
    if !path.exists(){panic!("path '{}' doesn't exist", path.display());}
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }


}
