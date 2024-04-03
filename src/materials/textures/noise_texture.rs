use nalgebra::vector;

use super::{Perlin, Texture};

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(256),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    #![allow(unused)]
    fn value(&self, u: f64, v: f64, point: nalgebra::Point3<f64>) -> nalgebra::Vector3<f64> {
        vector![1., 1., 1.] * 0.5 * (1. + f64::sin(self.scale * point.z + 10. * self.noise.turb(point, 7)))
    }
}
