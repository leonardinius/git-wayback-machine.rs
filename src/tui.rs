use std::error::Error;
use std::path::Path;

use rustbox;
use rustbox::{
    Color,
    Event,
    InitOptions,
    InputMode,
    Key,
    RustBox,
};

use history::{
    Entry,
    History
};

#[derive(Debug, Clone)]
pub enum TuiKey {
    Esc,
    Up,
    Down,
    PageUp,
    PageDown,
    Char(char),
    Ctrl(char),
}

#[derive(Debug, Clone)]
pub enum TuiEvent {
    UnSupported,
    Resize(usize, usize),
    KeyEvent(TuiKey),
}

pub struct TUI<'a> {
    rb : RustBox,
    history : History<'a>,
    page : usize,
    cursor: usize,
}

impl<'a> TUI<'a> {
    pub fn new(cwd: &'a Path) -> Self {
        let rustbox = Self::init_rustbox();
        let height = rustbox.height() as usize;

        TUI { page: 0, cursor: 0, rb: rustbox, history: History::new(height - 5, cwd) }
    }

    pub fn page(&self) -> usize { self.page }

    pub fn cursor(&self) -> usize { self.cursor }

    pub fn move_up(&mut self) -> &mut Self {
        if self.cursor() > 0 {
            self.cursor -= 1;
        } else {
            self.page_up();
            // TODO: optimize, redundant read
            self.cursor = self.get_page_size(self.page()) -1;
        }

        self.draw();

        self
    }

    pub fn move_down(&mut self) -> &mut Self {
            // TODO: optimize, redundant read
        if self.cursor() + 1 > self.get_page_size(self.page) {
            self.page_down();
            self.cursor = 0;
        } else {
            self.cursor += 1;
        }

        self.draw();

        self
    }

    pub fn page_up(&mut self) -> &mut Self {
        if self.page() > 0 {
            self.page -= 1;
        }

        self.draw();

        self
    }

    pub fn page_down(&mut self) -> &mut Self {
        if self.page() + 1 < self.get_page_count() {
            self.page += 1;
        }

        self.draw();

        self
    }

    pub fn resize(&mut self, height: usize) -> &mut Self {
        self.cursor = 0;
        self.page = 0;
        self.history.resize(height);

        self.draw();

        self
    }

    pub fn draw(&self) -> &Self {
        let rb = &self.rb;
        rb.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black,
                 &format!("{}/{} [{}]", self.page(), self.get_page_count(), self.history.cwd()));
        rb.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                      "Press 'q' to quit.");

        rb.present();

        self
    }

    pub fn poll_event(&self) -> TuiEvent {
        match self.rb.poll_event(false)
            .unwrap_or_else(|e|{ panic!("Failed poll rustbox event!. {}", e) })
        {
            Event::KeyEvent(Some(key)) => {
                match key {
                    Key::Esc        => TuiEvent::KeyEvent(TuiKey::Esc),
                    Key::Up         => TuiEvent::KeyEvent(TuiKey::Up),
                    Key::Down       => TuiEvent::KeyEvent(TuiKey::Down),
                    Key::PageUp     => TuiEvent::KeyEvent(TuiKey::PageUp),
                    Key::PageDown   => TuiEvent::KeyEvent(TuiKey::PageDown),
                    Key::Char(c)    => TuiEvent::KeyEvent(TuiKey::Char(c)),
                    Key::Ctrl(c)    => TuiEvent::KeyEvent(TuiKey::Ctrl(c)),
                    _ => TuiEvent::UnSupported,
                }
            },

            Event::ResizeEvent(width, height) => {
                TuiEvent::Resize(width as usize, height as usize)
            },

            _ => TuiEvent::UnSupported,
        }
    }

    fn get_page(&self, page: usize) -> Vec<Entry> {
        self.history.get_page(page).unwrap_or(vec![])
    }

    fn get_page_size(&self, page: usize) -> usize {
        self.history.get_page(page).map(|v| v.len() as usize).unwrap_or(0)
    }

    fn get_page_count(&self) -> usize {
        self.history.page_count().unwrap_or(0)
    }

    fn init_rustbox() -> RustBox {
        RustBox::init(InitOptions {
            buffer_stderr: true,
            input_mode: InputMode::Esc,
        })
        .unwrap_or_else(|e| {
            panic!("{}", e)
        })
    }
}
