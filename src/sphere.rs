use nalgebra::Point3;

use crate::{hittable::Hittable, interval::Interval, material::Material, ray::Ray};

use super::hit_record::HitRecord;

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, mat: &Material) -> Sphere {
        Sphere {
            center,
            radius,
            mat: mat.clone(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.contains(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let point = ray.at(root);

        Some(HitRecord::new(
            point,
            (point - self.center) / self.radius,
            &self.mat,
            root,
            ray,
        ))
    }
}
