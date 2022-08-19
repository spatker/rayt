
use crate::color::Color;

pub struct Image{
    width: usize,
    height: usize,
    data: Box<[Color]>,
}

use std::path::Path;
use std::io::Write;
use std::fs::File;

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width, height,
            data: vec![Color::new(0.0); width * height].into_boxed_slice()
        }
    }

    pub fn get_height_width(&self, i: usize) -> (usize, usize) {
        (i/self.width,i%self.width)
    }

    pub fn set_color(&mut self, height: usize, width: usize, c: Color) {
        self.data[width + self.width*height] = c;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn tonemap(&self) -> Vec<u8> {
        let mut data = vec![0; self.width * self.height * 3];
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
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write(header.as_bytes())?;
        let data = self.tonemap();
        file.write(&data)?;
        Ok(())
    }
}