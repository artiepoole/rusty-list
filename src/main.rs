use clap::Parser;
use env_logger;
use log::debug;
use std::path::PathBuf;
mod directory_printers;
mod directory_scrapers;
mod directory_sorters;

use directory_scrapers::{recursive_scraper::RecursiveScraper, shallow_scraper::scrape_directory};

use crate::directory_printers::simple::simple_printer;
use crate::directory_printers::tree_like::tree_like_printer;
use crate::directory_sorters::alphabetical::sort_purely_alphabetically;
use crate::directory_sorters::depth_based::{depth_alpha_sort, depth_only_sort};

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
    /// enable printing as if calling "tree"
    #[arg(short, long)]
    tree_like: bool,
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
    let mut all_paths = Vec::<PathBuf>::new();

    if args.recursive {
        let mut path_printer = RecursiveScraper {
            max_depth: args.depth,
            current_depth: 0,
        };
        path_printer
            .scrape_directory_recursive(&path, &mut all_paths)
            .expect(&format!(
                "Failed to print root dir '{:?}' recursively ",
                path
            ));
    } else {
        scrape_directory(&path, &mut all_paths)
            .expect(&format!("Failed to print root dir'{:?}'", path));
    }

    
    let mut form = 1;
    if args.tree_like {
        form = 2
    }
    match form {
        0 => { // pure alphabetical mode
            sort_purely_alphabetically(&mut all_paths);
            simple_printer(&all_paths);
        }
        1 => { // typical mode
            depth_alpha_sort(&mut all_paths);
            simple_printer(&all_paths);
        }
        2 => { // tree mode
            depth_alpha_sort(&mut all_paths);
            tree_like_printer(&all_paths)
        }
        3 => { // some whacky mode.
            depth_only_sort(&mut all_paths);
            simple_printer(&all_paths)
        }
        _ => { // fall back to unsorted simple
            simple_printer(&all_paths);
        }
    }
}
