use std::path::PathBuf;


static VERT: char = '│';
static TEE: char = '├';
static HOOK: char = '└';
static HORZ: char = '─';

pub fn tree_like_printer(all_paths: &Vec<PathBuf>) {
    // Fiddling with iterables: I got stuck trying to figure out how to even figure out how to get
    // anything meaningful from this mess: 
    // let prev_parent: PathBuf =  all_paths.first().unwrap().clone().components().enumerate();
    //
    // my algorithm thoughts:
    // 0) of course everything will start at '/'. Store this in "deepest_common"
    // 1) get the next component after "deepest_common" from any path and store this is "next_deepest"
    // 2) a) loop through all other paths and break if that path does not match up until "next_deepest"
    //    b) if all match, store next_deepest into deepest_common and go to 1)
    // 3) print deepest_common to start the tree printing
    // 4) get the next_deepest and store pointers to all paths matching this next_deepest
    // 5) ??? some sort of recursion to get all common files from each depth (should go the end and 
    //     work backwards? 
    // 6) print the appropriate characters :D
    // 
    // need to be able to store all indentation levels to the left of this line...  

    println!("WARN  tree like printing is WIP. Resorting to a simple print.");
    for path in all_paths {
        println!("{:?}", path);
    }
    println!("{}{}{}{}{}",VERT, TEE, HORZ, HOOK, HORZ)
    
}
