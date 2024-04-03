use std::sync::Arc;

use nalgebra::{point, vector};

use crate::{
    core::{HitRecord, Hittable, Ray},
    shapes::Aabb,
    utility::Interval,
};

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = object.bounding_box().clone();

        let mut min = point![f64::MAX, f64::MAX, f64::MAX];
        let mut max = point![f64::MIN, f64::MIN, f64::MIN];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1. - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1. - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1. - k as f64) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = vector![new_x, y, new_z];

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        bbox = Aabb::from_points(min, max);

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut origin = *ray.origin();
        let mut direction = *ray.direction();

        origin[0] = self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] = self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];

        direction[0] = self.cos_theta * ray.direction()[0] - self.sin_theta * ray.direction()[2];
        direction[2] = self.sin_theta * ray.direction()[0] + self.cos_theta * ray.direction()[2];

        let rotated_ray = Ray::with_time(origin, direction, *ray.time());

        let rec = self.object.hit(&rotated_ray, ray_t);

        if rec.is_none() {
            None
        } else {
            let mut rec = rec.unwrap();
            let mut p = rec.point;

            p.x = self.cos_theta * rec.point[0] + self.sin_theta * rec.point[2];
            p.z = -self.sin_theta * rec.point[0] + self.cos_theta * rec.point[2];

            let mut normal = rec.normal;
            normal.x = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal.z = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.point = p;
            rec.normal = normal;

            Some(rec)
        }
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
