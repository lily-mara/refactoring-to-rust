use image::{Rgb, RgbImage};
use num_complex::Complex64;
use pyo3::prelude::*;
use std::path::Path;

#[pyfunction]
fn mandelbrot_fast(
    py: Python<'_>,
    size: u32,
    path: str,
    range_x0: f64,
    range_y0: f64,
    range_x1: f64,
    range_y1: f64,
) {
    py.allow_threads(|| mandelbrot_func(size, path, range_x0, range_y0, range_x1, range_y1))
}
#[pyfunction]
fn mandelbrot_func(size: u32, p: str, range_x0: f64, range_y0: f64, range_x1: f64, range_y1: f64) {
    let mut img = RgbImage::new(size, size);

    let size_f64 = size as f64;

    let x_range = (range_x1 - range_x0).abs();
    let x_offset = x_range / 2.0;

    let y_range = (range_y1 - range_y0).abs();
    let y_offset = y_range / 2.0;
    let path = Path::new(p);
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
