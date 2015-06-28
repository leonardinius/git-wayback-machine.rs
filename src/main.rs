use std::env;
use std::path::PathBuf;

#[macro_use] extern crate log;
extern crate env_logger;
extern crate rustbox;

mod git;
mod history;
mod tui;

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
    let mut tui = TUI::new(&cwd);
    tui.draw();

    loop {
        match tui.poll_event() {
            TuiEvent::KeyEvent(TuiKey::Char('q')) | TuiEvent::KeyEvent(TuiKey::Char('Q'))  => {
                break;
            },

            TuiEvent::KeyEvent(TuiKey::Char('r')) | TuiEvent::KeyEvent(TuiKey::Char('R'))  => {
                tui.draw();
            },

            TuiEvent::KeyEvent(TuiKey::PageDown) => { tui.page_down(); tui.draw(); }
            TuiEvent::KeyEvent(TuiKey::PageUp) => { tui.page_up(); tui.draw(); }
            TuiEvent::KeyEvent(TuiKey::Down) => { tui.move_down(); tui.draw(); }
            TuiEvent::KeyEvent(TuiKey::Up) => { tui.move_up(); tui.draw(); }

            TuiEvent::Resize(_, y) => { tui.resize(y); tui.draw(); },

            TuiEvent::UnSupported => { /* ignore */ ; },

            _ => { /* ignore; do nothing */; },
        }
    }
}
