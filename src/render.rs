use crate::color::Color;
use crate::image::{Image, Resolution};

use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::fs::File;

impl Image {
    fn tonemap(&self) -> Vec<u8> {
        let mut data = vec![0; self.resolution.width * self.resolution.height * 3];
        for (i,c) in self.get_data().iter().enumerate() {
            let offset = i*3;

            // filmic tonemap formula by Jim Hejl and Richard Burgess-Dawson
            let x = Color{
                r: f32::max(0.0,c.r-0.004),
                g: f32::max(0.0,c.g-0.004),
                b: f32::max(0.0,c.b-0.004),
            };
            let color = (x*(6.2*x+0.5))/(x*(6.2*x+1.7)+0.06);

            data[offset] = (color.r*255.0) as u8;
            data[offset+1] = (color.g*255.0) as u8;
            data[offset+2] = (color.b*255.0) as u8;
        }
        data
    }

    pub fn save(&self, filename: &PathBuf) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.resolution.width, self.resolution.height);
        file.write(header.as_bytes())?;
        let data = self.tonemap();
        file.write(&data)?;
        Ok(())
    }
}