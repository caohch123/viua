pub mod ansi;

use crate::ascii::AsciiArt;
use std::io::{Result, Write};

pub trait Renderer<T> {
    fn render(&self, out: &mut impl Write, data: &T) -> Result<()>;
}

pub struct Ansi {
    pub color: bool,
    pub monochrome: bool,
}

impl Renderer<AsciiArt> for Ansi {
    fn render(&self, out: &mut impl Write, art: &AsciiArt) -> Result<()> {
        ansi::render(out, art, self.color, self.monochrome)
    }
}
