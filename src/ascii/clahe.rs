use image::{DynamicImage, GrayImage, Luma};

pub fn apply(img: &DynamicImage, tile_size: u32, clip_limit: f32) -> DynamicImage {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();

    let tiles_x = w.div_ceil(tile_size);
    let tiles_y = h.div_ceil(tile_size);

    if tiles_x < 2 || tiles_y < 2 {
        return DynamicImage::ImageLuma8(gray);
    }

    let mut cdfs: Vec<Vec<[u8; 256]>> = vec![vec![[0; 256]; tiles_x as usize]; tiles_y as usize];

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            let mut hist = [0u32; 256];
            let x0 = tx * tile_size;
            let y0 = ty * tile_size;
            let tw = tile_size.min(w - x0);
            let th = tile_size.min(h - y0);
            let pixel_count = tw * th;

            for dy in 0..th {
                for dx in 0..tw {
                    let p = gray.get_pixel(x0 + dx, y0 + dy);
                    hist[p[0] as usize] += 1;
                }
            }

            let clip = ((pixel_count as f32 / 256.0) * clip_limit).max(1.0) as u32;
            let mut excess = 0u32;
            for h in &mut hist {
                if *h > clip {
                    excess += *h - clip;
                    *h = clip;
                }
            }
            let redist = excess / 256;
            let remainder = excess % 256;
            for (i, h) in hist.iter_mut().enumerate() {
                *h += redist;
                if (i as u32) < remainder {
                    *h += 1;
                }
            }

            let mut cdf = 0u32;
            let mut lut = [0u8; 256];
            for (i, h) in hist.iter().enumerate() {
                cdf += h;
                lut[i] = ((cdf as f64 / pixel_count as f64) * 255.0).round() as u8;
            }
            cdfs[ty as usize][tx as usize] = lut;
        }
    }

    let half = (tile_size / 2) as f32;
    let mut out = GrayImage::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let fx = x as f32 - half;
            let fy = y as f32 - half;

            let tx = (fx / tile_size as f32).floor() as i32;
            let ty = (fy / tile_size as f32).floor() as i32;

            let tx0 = tx.clamp(0, tiles_x as i32 - 1) as usize;
            let ty0 = ty.clamp(0, tiles_y as i32 - 1) as usize;
            let tx1 = (tx + 1).clamp(0, tiles_x as i32 - 1) as usize;
            let ty1 = (ty + 1).clamp(0, tiles_y as i32 - 1) as usize;

            let wx = (fx - (tx as f32 * tile_size as f32)) / tile_size as f32;
            let wy = (fy - (ty as f32 * tile_size as f32)) / tile_size as f32;

            let p = gray.get_pixel(x, y)[0];

            let v00 = cdfs[ty0][tx0][p as usize] as f32;
            let v10 = cdfs[ty0][tx1][p as usize] as f32;
            let v01 = cdfs[ty1][tx0][p as usize] as f32;
            let v11 = cdfs[ty1][tx1][p as usize] as f32;

            let top = v00 + (v10 - v00) * wx;
            let bot = v01 + (v11 - v01) * wx;
            let val = (top + (bot - top) * wy).round() as u8;

            out.put_pixel(x, y, Luma([val]));
        }
    }

    DynamicImage::ImageLuma8(out)
}
