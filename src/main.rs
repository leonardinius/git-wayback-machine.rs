use std::env;
use std::path::PathBuf;

#[macro_use]
extern crate log;

mod git;

fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|e| {
        panic!("Get current dir expected to succeed. Error: {}", e);
    })
}

fn main() {
    match git::stash(&get_current_dir()) {
        Ok(stash) => println!("Stash: {}", stash),
        Err(e) => panic!("Error: {}", e),
    }

    match git::reset(&get_current_dir(), "HEAD") {
        Ok(reset) => println!("reset: {}", reset),
        Err(e) => panic!("Error: {}", e),
    }
}
