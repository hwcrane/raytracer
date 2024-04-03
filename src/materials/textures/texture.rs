use nalgebra::{Point3, Vector3};


pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, point: Point3<f64>) -> Vector3<f64>;
}


