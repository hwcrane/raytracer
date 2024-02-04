use std::sync::Arc;

use na::{Point3, Vector3};

use crate::{ray::Ray, material::Material};

pub struct HitRecord {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3<f64>, outward_normal: Vector3<f64>, mat: &Arc<dyn Material>,t: f64, ray: &Ray) -> HitRecord {
        let front_face = ray.direction().dot(&outward_normal) < 0.;
        HitRecord {
            point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            mat: mat.clone(),
            t,
            front_face,
        }
    }
}
