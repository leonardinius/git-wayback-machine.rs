#![feature(core)]

use std::env;
use std::path::PathBuf;

#[macro_use] extern crate log;
extern crate env_logger;
extern crate rustbox;

mod git;
mod history;
mod tui;

use history::{Entry, History};
use tui::{TUI, TuiEvent, TuiKey};

fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|e| {
        panic!("Get current dir expected to succeed. Error: {}", e);
    })
}

fn main() {
    env_logger::init().unwrap_or_else(|e| {
        panic!("Failed to init env_logger properly. Error: {}", e);
    });

    let cwd = get_current_dir();
    let tui = TUI::new(&cwd);
    tui.draw();

    loop {
        match tui.poll_event() {
            TuiEvent::KeyEvent(TuiKey::Char('q')) => {
                break;
            },

            TuiEvent::UnSupported => { /* ignore */ ; },

            e => { println!("Not yet supported: {:?}", e); },
        }
    }
}
