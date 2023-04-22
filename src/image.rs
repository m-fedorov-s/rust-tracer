#![allow(unused)]

use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct RGB(u8, u8, u8);

pub struct Image {
    pixels: Vec<Vec<RGB>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let mut column: Vec<RGB> = Vec::new();
        column.resize(height, RGB(255, 255, 255));
        let mut raw_data: Vec<Vec<RGB>> = Vec::new();
        raw_data.resize(width, column);
        Image { pixels: raw_data }
    }

    pub fn set_pixel(&mut self, column: usize, row: usize, value: &RGB) {
        self.pixels[column][row] = *value;
    }

    pub fn get_pixel(&self, column: usize, row: usize) -> RGB {
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

#[cfg(test)]
mod tests {
    use super::*;
}
