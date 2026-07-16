pub mod charset;
pub mod lum;

use image::DynamicImage;

pub enum Algorithm {
    Luminance,
}

pub struct AsciiPixel {
    pub char: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct AsciiArt {
    pub lines: Vec<Vec<AsciiPixel>>,
}

pub fn convert(img: &DynamicImage, new_width: u32, new_height: u32, char_set: &[char], algo: Algorithm) -> AsciiArt {
    match algo {
        Algorithm::Luminance => lum::convert(img, new_width, new_height, char_set),
    }
}
