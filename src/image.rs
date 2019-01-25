use png::{BitDepth, ColorType, Encoder, HasParameters};
use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;

use crate::vector::Vector3;

#[derive(Debug)]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let pixels = vec![0; (width * height * 3) as usize];
        Image {
            width,
            height,
            pixels,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Vector3) {
        let offset = 3 * (y * self.width + x) as usize;
        self.pixels[offset + 0] = (255.0 * color.x()) as u8;
        self.pixels[offset + 1] = (255.0 * color.y()) as u8;
        self.pixels[offset + 2] = (255.0 * color.z()) as u8;
    }

    pub fn save<T>(&self, path: T) -> Result<(), io::Error>
    where
        T: AsRef<Path>,
    {
        let file = File::create(path)?;
        let w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.width, self.height);
        encoder.set(ColorType::RGB).set(BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(self.pixels.as_slice())?;

        Ok(())
    }
}
