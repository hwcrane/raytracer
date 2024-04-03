use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    core::{HitRecord, Hittable, Ray},
    shapes::Aabb,
    utility::Interval,
};

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vector3<f64>,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vector3<f64>) -> Translate {
        let bbox = object.bounding_box() + offset;
        Translate {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::with_time(ray.origin() - self.offset, *ray.direction(), *ray.time());

        let rec = self.object.hit(&offset_ray, ray_t);
        match rec {
            Some(mut r) => {
                r.point += self.offset;
                Some(r)
            }
            None => None,
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
