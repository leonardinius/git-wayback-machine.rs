#![feature(associated_consts)]
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
        println!("History: {:?}", history.count());

        println!("\\------------------
                 History: {}
                 ", (&history.get_page_data(0).unwrap()[..]).connect("\n\n"));

        println!("\\------------------
                 History: {}
                 ", (&history.get_page_data(1).unwrap()[..]).connect("\n\n"));

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
