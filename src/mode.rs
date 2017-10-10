use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy)]
pub enum Mode {
    Insert,
    Normal,
    // Replace,
    // Visual,
    // VisualLine,
    // VisualBlock,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Mode::Insert => write!(f, "INSERT"),
            Mode::Normal => write!(f, "NORMAL"),
            // Mode::Replace => write!(f, "REPLACE"),
            // Mode::Visual => write!(f, "VISUAL"),
            // Mode::VisualLine => write!(f, "VISUAL-LINE"),
            // Mode::VisualBlock => write!(f, "VISUAL-BLOCK"),
        }
    }
}
