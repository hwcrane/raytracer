use image::{ImageResult, DynamicImage, GenericImageView};
use nalgebra::{vector, Point3, Vector3};

use crate::interval::Interval;

use super::Texture;


pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageResult<ImageTexture> {
        let image = image::io::Reader::open(filename)?.decode()?;
        Ok(ImageTexture { image })
    }
}

impl Texture for ImageTexture {
    #![allow(unused)]
    fn value(&self, u: f64, v: f64, point: Point3<f64>) -> Vector3<f64> {
        let u = Interval::new(0., 1.).clamp(u);
        let v = 1. - Interval::new(0., 1.).clamp(v);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        let pixel = self.image.get_pixel(i, j);

        let colour_scale = 1. / 255.;
        vector![
            colour_scale * pixel[0] as f64,
            colour_scale * pixel[1] as f64,
            colour_scale * pixel[2] as f64
        ]
    }
}

