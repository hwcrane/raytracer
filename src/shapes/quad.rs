use nalgebra::{point, vector, Point3, Vector3};

use crate::{core::{HitRecord, Hittable, HittableList, Ray}, materials::Material, utility::Interval};

use super::Aabb;

pub struct Quad {
    q: Point3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    mat: Material,
    bbox: Aabb,
    normal: Vector3<f64>,
    d: f64,
    w: Vector3<f64>,
}

impl Quad {
    pub fn new(q: Point3<f64>, u: Vector3<f64>, v: Vector3<f64>, mat: &Material) -> Quad {
        let bbox = Aabb::from_points(q, q + u + v).pad();
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&vector![q.x, q.y, q.z]);
        let w = n / n.dot(&n);
        Quad {
            q,
            u,
            v,
            mat: mat.clone(),
            bbox,
            normal,
            d,
            w,
        }
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        ray: &Ray,
        ray_t: Interval,
    ) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction());

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d
            - self
                .normal
                .dot(&vector![ray.origin().x, ray.origin().y, ray.origin().z]))
            / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if alpha < 0. || 1. < alpha || beta < 0. || 1. < beta {
            return None;
        }

        Some(HitRecord::new(
            intersection,
            self.normal,
            &self.mat,
            t,
            alpha,
            beta,
            ray,
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub fn make_box(a: Point3<f64>, b: Point3<f64>, mat: &Material) -> HittableList {
    let mut sides = HittableList::new();

    let min = point![a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)];
    let max = point![a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)];

    let dx = vector![max.x - min.x, 0., 0.];
    let dy = vector![0., max.y - min.y, 0.];
    let dz = vector![0., 0., max.z - min.z];

    sides.add(Box::new(Quad::new(
        point![min.x, min.y, max.z],
        dx,
        dy,
        mat,
    ))); // front
    sides.add(Box::new(Quad::new(
        point![max.x, min.y, max.z],
        -dz,
        dy,
        mat,
    ))); // right
    sides.add(Box::new(Quad::new(
        point![max.x, min.y, min.z],
        -dx,
        dy,
        mat,
    ))); // back
    sides.add(Box::new(Quad::new(
        point![min.x, min.y, min.z],
        dz,
        dy,
        mat,
    ))); // left
    sides.add(Box::new(Quad::new(
        point![min.x, max.y, max.z],
        dx,
        -dz,
        mat,
    ))); // top
    sides.add(Box::new(Quad::new(
        point![min.x, min.y, min.z],
        dx,
        dy,
        mat,
    ))); // top

    sides
}
