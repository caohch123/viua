use crate::ascii::AsciiArt;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::queue;
use std::io::{Result, Write};

pub fn render<W: Write>(out: &mut W, art: &AsciiArt, color: bool, monochrome: bool) -> Result<()> {
    for row in &art.lines {
        for pixel in row {
            if color && !monochrome {
                let r = pixel.r;
                let g = pixel.g;
                let b = pixel.b;
                queue!(out, SetForegroundColor(Color::Rgb { r, g, b }))?;
            }
            queue!(out, Print(pixel.char))?;
        }
        if color && !monochrome {
            queue!(out, ResetColor)?;
        }
        queue!(out, Print("\r\n"))?;
    }
    Ok(())
}
