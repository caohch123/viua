use crate::ascii::AsciiArt;
use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ascii::{AsciiArt, AsciiPixel};
    use std::io::Cursor;

    fn make_art() -> AsciiArt {
        AsciiArt {
            lines: vec![vec![AsciiPixel {
                char: 'X',
                r: 255,
                g: 128,
                b: 64,
            }]],
        }
    }

    #[test]
    fn test_render_monochrome_no_color_codes() {
        let art = make_art();
        let mut buf = Cursor::new(Vec::new());
        render(&mut buf, &art, false, false).unwrap();
        let output = String::from_utf8(buf.into_inner()).unwrap();
        assert!(!output.contains("\x1b[38;2;"));
        assert!(output.contains('X'));
        assert!(output.contains("\r\n"));
    }

    #[test]
    fn test_render_color_emits_rgb_escape() {
        let art = make_art();
        let mut buf = Cursor::new(Vec::new());
        render(&mut buf, &art, true, false).unwrap();
        let output = String::from_utf8(buf.into_inner()).unwrap();
        assert!(output.contains("\x1b[38;2;255;128;64m"));
        assert!(output.contains("X"));
    }
}
