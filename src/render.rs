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

pub fn render_html<W: Write>(out: &mut W, art: &AsciiArt, title: &str) -> Result<()> {
    writeln!(out, "<!DOCTYPE html>")?;
    writeln!(out, "<html lang=\"en\">")?;
    writeln!(out, "<head>")?;
    writeln!(out, "  <meta charset=\"UTF-8\">")?;
    writeln!(out, "  <title>{}</title>", html_escape(title))?;
    writeln!(out, "  <style>")?;
    writeln!(out, "    body {{ background: #1e1e1e; margin: 0; padding: 1rem; }}")?;
    writeln!(out, "    pre {{ font-family: monospace; font-size: 12px; line-height: 1.2; }}")?;
    writeln!(out, "  </style>")?;
    writeln!(out, "</head>")?;
    writeln!(out, "<body><pre>")?;

    for row in &art.lines {
        for pixel in row {
            let escaped = html_escape_char(pixel.char);
            write!(
                out,
                "<span style=\"color:rgb({},{},{})\">{}",
                pixel.r, pixel.g, pixel.b, escaped
            )?;
            write!(out, "</span>")?;
        }
        writeln!(out)?;
    }

    writeln!(out, "</pre></body>")?;
    writeln!(out, "</html>")?;
    Ok(())
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn html_escape_char(c: char) -> String {
    match c {
        '&' => "&amp;".to_string(),
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '"' => "&quot;".to_string(),
        _ => c.to_string(),
    }
}
