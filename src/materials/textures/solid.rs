use nalgebra::{vector, Point3, Vector3};

use super::Texture;

pub struct SolidColour {
    colour: Vector3<f64>,
}

impl SolidColour {
    pub fn new(colour: Vector3<f64>) -> SolidColour {
        SolidColour { colour }
    }
    pub fn from_rgb(r: f64, g: f64, b: f64) -> SolidColour {
        SolidColour {
            colour: vector![r, g, b],
        }
    }
}

impl Texture for SolidColour {
    #![allow(unused)]
    fn value(&self, u: f64, v: f64, point: Point3<f64>) -> Vector3<f64> {
        self.colour
    }
}
