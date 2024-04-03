use std::sync::Arc;


use crate::{shapes::Aabb, utility::Interval};

use super::{hit_record::HitRecord, Hittable, Ray};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![], bbox: Aabb::default() }
    }

    pub fn add_list(&mut self, objects: HittableList) {
        for obj in objects.objects {
            self.bbox = self.bbox.merge(&obj.bounding_box());
            self.objects.push(obj);
        }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = self.bbox.merge(&object.bounding_box());
        self.objects.push(object.into());
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut best_rec = None;
        let mut hit_anything = false;
        let mut closest = ray_t;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(ray, closest) {
                hit_anything = true;
                closest.max = rec.t;
                best_rec = Some(rec);
            }
        }

        if hit_anything {
            best_rec
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
