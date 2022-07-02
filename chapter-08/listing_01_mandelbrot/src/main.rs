use std::path::Path;

use image::{Rgb, RgbImage};
use num_complex::Complex64;

fn main() {
  mandelbrot(1000, "mandelbrot_purple.png", -5.0, -2.12, -2.5, 1.12);
}

fn mandelbrot(
  size: u32,
  path: impl AsRef<Path>,
  range_x0: f64,
  range_y0: f64,
  range_x1: f64,
  range_y1: f64,
) {
  let mut img = RgbImage::new(size, size);

  let size_f64 = size as f64;

  let x_range = (range_x1 - range_x0).abs();
  let x_offset = x_range / 2.0;

  let y_range = (range_y1 - range_y0).abs();
  let y_offset = y_range / 2.0;

  for px in 0..size {
    for py in 0..size {
      let x0 = px as f64 / size_f64 * x_range - x_offset;
      let y0 = py as f64 / size_f64 * y_range - y_offset;

      let c = Complex64::new(x0, y0);

      let mut i = 0u8;
      let mut z = Complex64::new(0.0, 0.0);

      while i < 255 {
        z = (z * z) + c;
        if z.norm() > 4.0 {
          break;
        }
        i += 1;
      }

      img.put_pixel(px, py, Rgb([i, i, i]));
    }
  }

  img.save(path).unwrap();
}
