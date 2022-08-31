use crate::color::Color;
use std::fs::File;
use std::path::PathBuf;

pub struct Image{
    pub resolution: Resolution,
    data: Vec<Color>,
}

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

pub struct ImageError {}

impl From<std::io::Error> for ImageError {   
    fn from(_: std::io::Error) -> Self { ImageError {} }
}

impl From<hdrldr::LoadError> for ImageError {   
    fn from(_: hdrldr::LoadError) -> Self { ImageError {} }
}

impl Resolution {
    pub fn new(width: usize, height: usize) -> Resolution {
        Resolution{width, height}
    }

    pub fn get_height_width(&self, i: usize) -> (usize, usize) {
        (self.width - i/self.width,i%self.width)
    }

    pub fn get_index(&self, width: usize, height: usize) -> usize {
        usize::min(height * self.width + width, (self.width * self.height) -1 )
    }
}

impl Image {
    pub fn new(resolution: Resolution) -> Image {
        Image {
            resolution,
            data: vec![Color::default(); resolution.width * resolution.height]
        }
    }
    
    pub fn get_data(&self) -> &Vec<Color> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<Color> {
        &mut self.data
    }

    pub fn load(filename: &PathBuf) -> Result<Image, ImageError> {
        let hdr = hdrldr::load(File::open(filename)?)?;
        let resolution = Resolution{width: hdr.width, height: hdr.height};
        let mut image = Image::new(resolution);
        hdr.data.iter().enumerate().for_each(|(i, p)| {
            image.data[i] = Color{r: p.r, g: p.g, b: p.b}
        });
        Ok(image)
    }
}