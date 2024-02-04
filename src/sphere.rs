use std::sync::Arc;

use nalgebra::Point3;

use crate::{interval::Interval, ray::Ray, material::Material};

use super::{hit_record::HitRecord, hittable_enum::Hittable, hittable_trait::HittableTrait};

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    mat: Arc<Material>
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, mat: Arc<Material>) -> Sphere {
        Sphere { center, radius, mat}
    }

    pub fn wrap_hittable(self) -> Hittable {
        Hittable::Sphere(self)
    }

    pub fn new_wrapped(center: Point3<f64>, radius: f64, mat: Arc<Material>) -> Hittable {
        Hittable::Sphere(Sphere { center, radius, mat })
    }
}

impl HittableTrait for Sphere {
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
