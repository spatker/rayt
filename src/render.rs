
use crate::color::Color;

pub struct Image{
    resolution: Resolution,
    data: Box<[Color]>,
}

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

impl Resolution {
    pub fn new(width: usize, height: usize) -> Resolution {
        Resolution{width, height}
    }

    pub fn get_height_width(&self, i: usize) -> (usize, usize) {
        (i/self.width,i%self.width)
    }
}

use std::path::Path;
use std::io::Write;
use std::fs::File;

impl Image {
    pub fn new(resolution: Resolution) -> Image {
        Image {
            resolution,
            data: vec![Color::new(0.0); resolution.width * resolution.height].into_boxed_slice()
        }
    }

    pub fn set_color(&mut self, height: usize, width: usize, c: Color) {
        self.data[width + self.resolution.width*height] = c;
    }

    pub fn get_resolution(&self) -> Resolution {
        self.resolution
    }

    pub fn get_data(&mut self) -> &mut Box<[Color]> {
        &mut self.data
    }

    fn tonemap(&self) -> Vec<u8> {
        let mut data = vec![0; self.resolution.width * self.resolution.height * 3];
        for (i,c) in self.data.iter().enumerate() {
            let offset = i*3;
            data[offset] = (c.r*255.0) as u8;
            data[offset+1] = (c.g*255.0) as u8;
            data[offset+2] = (c.b*255.0) as u8;
        }
        data
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.resolution.width, self.resolution.height);
        file.write(header.as_bytes())?;
        let data = self.tonemap();
        file.write(&data)?;
        Ok(())
    }
}