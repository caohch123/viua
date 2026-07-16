use super::{AsciiArt, AsciiPixel};
use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma};

fn sobel_magnitude(img: &GrayImage) -> GrayImage {
    let (w, h) = img.dimensions();
    let mut out = ImageBuffer::new(w, h);

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let gx = -(img.get_pixel(x - 1, y - 1)[0] as i32)
                + img.get_pixel(x + 1, y - 1)[0] as i32
                - 2 * img.get_pixel(x - 1, y)[0] as i32
                + 2 * img.get_pixel(x + 1, y)[0] as i32
                - img.get_pixel(x - 1, y + 1)[0] as i32
                + img.get_pixel(x + 1, y + 1)[0] as i32;

            let gy = -(img.get_pixel(x - 1, y - 1)[0] as i32)
                - 2 * img.get_pixel(x, y - 1)[0] as i32
                - img.get_pixel(x + 1, y - 1)[0] as i32
                + img.get_pixel(x - 1, y + 1)[0] as i32
                + 2 * img.get_pixel(x, y + 1)[0] as i32
                + img.get_pixel(x + 1, y + 1)[0] as i32;

            let mag = ((gx * gx + gy * gy) as f64).sqrt().round() as u8;
            out.put_pixel(x, y, Luma([mag]));
        }
    }

    out
}

pub fn convert(img: &DynamicImage, new_width: u32, new_height: u32, char_set: &[char]) -> AsciiArt {
    let gray = img.to_luma8();
    let edges = sobel_magnitude(&gray);
    let edges_rgb = DynamicImage::ImageLuma8(edges);
    let resized = edges_rgb.resize_exact(new_width, new_height, image::imageops::Lanczos3);

    let orig_resized = img.resize_exact(new_width, new_height, image::imageops::Lanczos3);

    let mut lines = Vec::new();
    for y in 0..new_height {
        let mut row = Vec::new();
        for x in 0..new_width {
            let edge_pixel = resized.get_pixel(x, y);
            let edge_val = edge_pixel[0];

            let inv: u8 = 255 - edge_val;
            let idx = (inv as usize) * (char_set.len() - 1) / 255;
            let ch = char_set[idx.min(char_set.len() - 1)];

            let orig_pixel = orig_resized.get_pixel(x, y);
            row.push(AsciiPixel {
                char: ch,
                r: orig_pixel[0],
                g: orig_pixel[1],
                b: orig_pixel[2],
            });
        }
        lines.push(row);
    }

    AsciiArt { lines }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    fn test_charset() -> Vec<char> {
        crate::ascii::charset::DEFAULT_CHARSET.chars().collect()
    }

    #[test]
    fn test_sobel_output_dimensions() {
        let img = DynamicImage::ImageRgba8(RgbaImage::new(100, 100));
        let art = convert(&img, 50, 25, &test_charset());
        assert_eq!(art.lines.len(), 25);
        assert_eq!(art.lines[0].len(), 50);
    }

    #[test]
    fn test_sobel_black_image_no_edges() {
        let img =
            DynamicImage::ImageRgba8(RgbaImage::from_pixel(10, 10, image::Rgba([0, 0, 0, 255])));
        let art = convert(&img, 10, 5, &test_charset());
        assert_eq!(art.lines[0][0].char, '@');
    }

    #[test]
    fn test_sobel_white_image_no_edges() {
        let img = DynamicImage::ImageRgba8(RgbaImage::from_pixel(
            10,
            10,
            image::Rgba([255, 255, 255, 255]),
        ));
        let art = convert(&img, 10, 5, &test_charset());
        assert_eq!(art.lines[0][0].char, '@');
    }

    #[test]
    fn test_sobel_edge_detects_contrast() {
        let mut img = RgbaImage::new(20, 20);
        for y in 0..20 {
            for x in 0..20 {
                img.put_pixel(x, y, image::Rgba([if x < 10 { 0 } else { 255 }, 0, 0, 255]));
            }
        }
        let dyn_img = DynamicImage::ImageRgba8(img);
        let art = convert(&dyn_img, 20, 10, &test_charset());
        let mid_char = art.lines[5][10].char;
        assert!(
            mid_char != '@',
            "edge column should have a darker char than solid area"
        );
    }
}
