use nalgebra::{Point3, Vector3};

use crate::{core::Ray, utility::Interval};


#[derive(Default, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl std::ops::Add<Vector3<f64>> for Aabb {
    type Output = Self;
    fn add(self, rhs: Vector3<f64>) -> Self::Output {
        Aabb::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl std::ops::Add<Aabb> for Vector3<f64> {
    type Output = Aabb;
    fn add(self, rhs: Aabb) -> Self::Output {
        rhs + self
    }
}
impl std::ops::Add<Vector3<f64>> for &Aabb {
    type Output = Aabb;
    fn add(self, rhs: Vector3<f64>) -> Self::Output {
        Aabb::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl std::ops::Add<&Aabb> for Vector3<f64> {
    type Output = Aabb;
    fn add(self, rhs: &Aabb) -> Self::Output {
        rhs + self
    }
}

impl Aabb {
    pub fn new(ix: Interval, iy: Interval, iz: Interval) -> Aabb {
        Aabb {
            x: ix,
            y: iy,
            z: iz,
        }
    }
    pub fn from_points(a: Point3<f64>, b: Point3<f64>) -> Aabb {
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));
        Aabb { x, y, z }
    }
    pub fn merge(&self, bbox: &Aabb) -> Aabb {
        let x = Interval::merge(self.x, bbox.x);
        let y = Interval::merge(self.y, bbox.y);
        let z = Interval::merge(self.z, bbox.z);
        Aabb { x, y, z }
    }

    pub fn pad(&self) -> Aabb {
        let delta = 0.0001;
        let x = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };
        let y = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };
        let z = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };
        Aabb { x, y, z }
    }

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        let mut ray_t = ray_t;
        for a in 0..3 {
            let inv_d = 1. / ray.direction()[a];
            let orig = ray.origin()[a];

            let mut t0 = (self.axis(a).min - orig) * inv_d;
            let mut t1 = (self.axis(a).max - orig) * inv_d;

            if inv_d < 0. {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.min {
                ray_t.min = t0
            }
            if t1 < ray_t.max {
                ray_t.max = t1
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}
