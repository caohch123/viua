use super::{AsciiArt, AsciiPixel};
use image::{DynamicImage, GenericImageView, Pixel};

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
            let gray =
                (0.299 * rgb[0] as f64 + 0.587 * rgb[1] as f64 + 0.114 * rgb[2] as f64) as u8;
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    fn test_charset() -> Vec<char> {
        crate::ascii::charset::DEFAULT_CHARSET.chars().collect()
    }

    #[test]
    fn test_convert_output_dimensions() {
        let img = DynamicImage::ImageRgba8(RgbaImage::new(100, 100));
        let art = convert(&img, 50, &test_charset());
        assert_eq!(art.lines.len(), 25);
        assert_eq!(art.lines[0].len(), 50);
    }

    #[test]
    fn test_convert_black_to_first_char() {
        let img = DynamicImage::ImageRgba8(RgbaImage::from_pixel(10, 10, Rgba([0, 0, 0, 255])));
        let art = convert(&img, 10, &test_charset());
        assert_eq!(art.lines[0][0].char, ' ');
    }

    #[test]
    fn test_convert_white_to_last_char() {
        let img =
            DynamicImage::ImageRgba8(RgbaImage::from_pixel(10, 10, Rgba([255, 255, 255, 255])));
        let art = convert(&img, 10, &test_charset());
        assert_eq!(art.lines[0][0].char, '@');
    }

    #[test]
    fn test_convert_preserves_rgb() {
        let img =
            DynamicImage::ImageRgba8(RgbaImage::from_pixel(10, 10, Rgba([100, 150, 200, 255])));
        let art = convert(&img, 10, &test_charset());
        let p = &art.lines[0][0];
        assert_eq!(p.r, 100);
        assert_eq!(p.g, 150);
        assert_eq!(p.b, 200);
    }
}
