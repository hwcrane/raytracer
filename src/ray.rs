use nalgebra::{Point3, Vector3};

pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    /// Creates a new [`Ray`].
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {origin, direction}
    }

    /// Returns the position of this [`Ray`] at distance `t`.
    pub fn at(&self, t: f64) -> Point3<f64>{
        self.origin + t * self.direction
    }

    /// Returns a reference to the direction of this [`Ray`].
    pub fn direction(&self) -> &Vector3<f64> {
        &self.direction
    }

    /// Returns a reference to the origin of this [`Ray`].
    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }
}
