#![allow(unused)]

use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct Rgb(u8, u8, u8);

pub struct RgbImage {
    pixels: Vec<Vec<Rgb>>,
}

pub struct LinearImage {
    pixels: Vec<Vec<(f64, f64, f64)>>,
}

impl RgbImage {
    pub fn new(width: usize, height: usize) -> RgbImage {
        let mut column: Vec<Rgb> = Vec::new();
        column.resize(height, Rgb(255, 255, 255));
        let mut raw_data: Vec<Vec<Rgb>> = Vec::new();
        raw_data.resize(width, column);
        RgbImage { pixels: raw_data }
    }

    pub fn set_pixel(&mut self, column: usize, row: usize, value: &Rgb) {
        self.pixels[column][row] = *value;
    }

    pub fn get_pixel(&self, column: usize, row: usize) -> Rgb {
        self.pixels[column][row]
    }

    pub fn write_to(&self, path: &str) -> Result<(), png::EncodingError> {
        let path = Path::new(path);
        let file = File::create(path)?;
        let w = &mut BufWriter::new(file);
        let height: u32 = self.pixels[0].len().try_into().unwrap();
        let width: u32 = self.pixels.len().try_into().unwrap();
        let mut encoder = png::Encoder::new(w, width, height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data = self.to_slice();
        writer.write_image_data(&data[..])
    }

    fn to_slice(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        let height = self.pixels[0].len();
        let width = self.pixels.len();
        result.reserve(height * width * 3);
        for row in 0..height {
            for column in 0..width {
                result.push(self.pixels[column][row].0);
                result.push(self.pixels[column][row].1);
                result.push(self.pixels[column][row].2);
            }
        }
        result
    }
}

impl LinearImage {
    pub fn new(width: usize, height: usize) -> LinearImage {
        let mut column: Vec<(f64, f64, f64)> = Vec::new();
        column.resize(height, (0.0, 0.0, 0.0));
        let mut raw_data: Vec<Vec<(f64, f64, f64)>> = Vec::new();
        raw_data.resize(width, column);
        LinearImage { pixels: raw_data }
    }

    pub fn set_pixel(&mut self, column: usize, row: usize, value: &(f64, f64, f64)) {
        self.pixels[column][row] = *value;
    }

    pub fn get_pixel(&self, column: usize, row: usize) -> (f64, f64, f64) {
        self.pixels[column][row]
    }

    pub fn convert_to_rgb(&self) -> RgbImage {
        // TODO Make gamma correction
        // See https://en.wikipedia.org/wiki/Gamma_correction and https://www.scratchapixel.com/lessons/digital-imaging/digital-images
        panic!("Not implemented!")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}
