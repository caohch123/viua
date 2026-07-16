pub mod charset;
pub mod clahe;
pub mod lum;

use image::DynamicImage;

#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    Luminance,
    LuminanceClahe,
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

pub fn convert(
    img: &DynamicImage,
    new_width: u32,
    new_height: u32,
    char_set: &[char],
    algo: Algorithm,
) -> AsciiArt {
    let img = match algo {
        Algorithm::Luminance => img.clone(),
        Algorithm::LuminanceClahe => clahe::apply(img, 8, 2.0),
    };
    lum::convert(&img, new_width, new_height, char_set)
}
