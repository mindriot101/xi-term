use std::io::Write;

use termion::style::{Invert, NoInvert};
use termion::cursor::Goto;
use termion::clear::CurrentLine as ClearLine;

use mode::Mode;

pub struct StatusBar {
    position: u16,
    width: u16,
    text: String,
    mode: Mode,
}

impl StatusBar {
    pub fn new() -> Self {
        StatusBar {
            text: "".into(),
            position: 0,
            width: 0,
            mode: Mode::Normal,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set(&mut self, text: String) {
        self.text = text;
    }

    pub fn clear(&mut self) {
        self.text = "".into();
    }
    pub fn set_position(&mut self, position: u16) {
        info!("setting position: {}", position);
        self.position = position;
    }

    pub fn set_width(&mut self, width: u16) {
        info!("setting width: {}", width);
        self.width = width;
    }

    pub fn render<W: Write>(&mut self, w: &mut W) {
        let res = write!(
            w,
            "{}{}{}{}{}{}{}{}{}-- {} --",
            Goto(1, self.position),
            ClearLine,
            Invert,
            &self.pad(),
            &self.text,
            Goto(1, self.position),
            Goto(self.width, self.position),
            NoInvert,
            Goto(1, self.position + 1),
            self.mode
        );
        if let Err(e) = res {
            error!("failed to render status bar: {}", e);
        }
    }

    fn pad(&self) -> String {
        " ".repeat(self.width as usize)
    }
}
