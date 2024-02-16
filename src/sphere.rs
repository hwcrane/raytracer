use std::{f64::consts::PI, sync::Arc};

use nalgebra::{vector, Point3, Vector3};

use crate::{aabb::Aabb, hittable::Hittable, interval::Interval, material::Material, ray::Ray};

use super::hit_record::HitRecord;

pub struct Sphere {
    center1: Point3<f64>,
    radius: f64,
    mat: Material,
    center_vec: Option<Vector3<f64>>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, mat: &Material) -> Sphere {
        let rvec = vector![radius, radius, radius];
        let bbox = Aabb::from_points(center + rvec, center - rvec);
        Sphere {
            center1: center,
            radius,
            mat: mat.clone(),
            center_vec: None,
            bbox,
        }
    }
    pub fn new_moving(
        center1: Point3<f64>,
        center2: Point3<f64>,
        radius: f64,
        mat: &Material,
    ) -> Sphere {
        let rvec = vector![radius, radius, radius];
        let bbox1 = Aabb::from_points(center1 - rvec, center1 + rvec);
        let bbox2 = Aabb::from_points(center2 - rvec, center2 + rvec);
        Sphere {
            center1,
            radius,
            mat: mat.clone(),
            center_vec: Some(center2 - center1),
            bbox: Aabb::merge(&bbox1, &bbox2),
        }
    }

    pub fn get_uv(&self, point: Vector3<f64>) -> (f64, f64) {
        let theta = f64::acos(-point.y);
        let phi = f64::atan2(-point.z, point.x) + PI;

        (phi / (2. * PI), theta / PI)
    }

    fn center(&self, time: f64) -> Point3<f64> {
        match &self.center_vec {
            Some(vec) => self.center1 + time * vec,
            None => self.center1,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(*ray.time());
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
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center1) / self.radius;
        let (u, v) = self.get_uv(outward_normal);

        Some(HitRecord::new(
            point,
            outward_normal,
            &self.mat,
            root,
            u,
            v,
            ray,
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
