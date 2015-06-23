use std::env;
use std::path::Path;

#[macro_use]
extern crate log;

mod git;

fn get_current_dir() -> Path {
    env::current_dir().as_path()
}

fn main() {
    match git::stash(get_current_dir()) {
        Ok(stash) => println!("Stash: {}", stash),
        Err(e) => panic!("Error: {}", e),
    }
}
