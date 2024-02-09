use std::sync::Arc;

use nalgebra::{Point3, Vector3};

use super::{SolidColour, Texture};

pub struct Checker {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}
impl Checker {
    pub fn new(scale: f64, even: &Arc<dyn Texture>, odd: &Arc<dyn Texture>) -> Checker {
        Checker {
            inv_scale: 1. / scale,
            even: even.clone(),
            odd: odd.clone(),
        }
    }
    pub fn from_colours(scale: f64, even: Vector3<f64>, odd: Vector3<f64>) -> Checker {
        Checker {
            inv_scale: 1. / scale,
            even: Arc::new(SolidColour::new(even)),
            odd: Arc::new(SolidColour::new(odd)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, point: Point3<f64>) -> Vector3<f64> {
        let x = (self.inv_scale * point.x).floor() as i32;
        let y = (self.inv_scale * point.y).floor() as i32;
        let z = (self.inv_scale * point.z).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
