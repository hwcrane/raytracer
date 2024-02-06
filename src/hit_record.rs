use na::{Point3, Vector3};

use crate::{material::Material, ray::Ray};

pub struct HitRecord<'a> {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub mat: &'a Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn new<'a>(
        point: Point3<f64>,
        outward_normal: Vector3<f64>,
        mat: &'a Material,
        t: f64,
        ray: &Ray,
    ) -> HitRecord<'a> {
        let front_face = ray.direction().dot(&outward_normal) < 0.;
        HitRecord {
            point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            mat,
            t,
            front_face,
        }
    }
}
