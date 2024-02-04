use nalgebra::Vector3;
use rand::Rng;

pub fn rng_vec() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen(), rng.gen(), rng.gen())
}

pub fn rng_vec_bound(min: f64, max: f64) -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    Vector3::new(
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
    )
}

pub fn rng_unit_sphere() -> Vector3<f64> {
    let mut p = rng_vec_bound(-1., 1.);
    while p.norm_squared() < 1. {
        p = rng_vec_bound(-1., 1.);
    }
    p
}

pub fn rng_unit_vec() -> Vector3<f64> {
    rng_unit_sphere().normalize()
}

pub fn rng_on_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let on_sphere = rng_unit_vec();
    if on_sphere.dot(normal) > 0. {
        on_sphere
    } else {
        -on_sphere
    }
}
