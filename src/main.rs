use std::env;
use std::path::PathBuf;

#[macro_use]
extern crate log;
extern crate env_logger;

mod git;
mod history;

use history::{Entry, History};

fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|e| {
        panic!("Get current dir expected to succeed. Error: {}", e);
    })
}

fn main() {
    env_logger::init().unwrap_or_else(|e| {
        panic!("Failed to init env_logger properly. Error: {}", e);
    });

    let cwd = &get_current_dir();
    {
        let history = History::new(4, cwd);
        println!("Entries: {:?}", history.entries_count());

        for i in 0..history.page_count().unwrap_or(0) {
            let page = history.get_page(i);
            println!("Page {}: {:?}", i + 1, page);
        }

        // match git::stash(cwd) {
        //     Ok(stash) => println!("Stash: {}", stash),
        //     Err(e) => panic!("Error: {}", e),
        // } 

        // match git::reset(cwd, "HEAD") {
        //     Ok(reset) => println!("reset: {}", reset),
        //     Err(e) => panic!("Error: {}", e),
        // }
    }
}
