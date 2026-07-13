use image::{DynamicImage, GenericImageView, Pixel};

pub struct AsciiPixel {
    pub char: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct AsciiArt {
    pub lines: Vec<Vec<AsciiPixel>>,
}

pub fn convert(img: &DynamicImage, new_width: u32, char_set: &[char]) -> AsciiArt {
    let (w, h) = img.dimensions();
    let aspect = h as f64 / w as f64;
    let new_height = (new_width as f64 * aspect * 0.5).round() as u32;
    let resized = img.resize_exact(new_width, new_height, image::imageops::Lanczos3);

    let mut lines = Vec::new();
    for y in 0..new_height {
        let mut row = Vec::new();
        for x in 0..new_width {
            let pixel = resized.get_pixel(x, y);
            let rgb = pixel.to_rgb();
            let gray = (0.299 * rgb[0] as f64 + 0.587 * rgb[1] as f64 + 0.114 * rgb[2] as f64) as u8;
            let idx = (gray as usize) * (char_set.len() - 1) / 255;
            let ch = char_set[idx.min(char_set.len() - 1)];
            row.push(AsciiPixel {
                char: ch,
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            });
        }
        lines.push(row);
    }

    AsciiArt { lines }
}
