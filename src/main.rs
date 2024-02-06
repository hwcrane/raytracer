pub mod camera;
pub mod hit_record;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod random;
pub mod ray;
pub mod sphere;
pub mod scenes;

extern crate nalgebra as na;

use camera::CameraConfig;
use na::point;
use na::vector;
pub use na::{Point3, Vector3};

fn main() {
    // Camera
    let cam = CameraConfig {
        aspect_ratio: 16. / 9.,
        image_width: 1200,
        samples_per_pixel: 500,
        max_depth: 50,
        vfov: 20.,
        lookfrom: point![13., 2., 3.],
        lookat: point![0., 0.,0.],
        vup: vector![0., 1., 0.],
        defocus_angle: 0.6,
        focus_dist: 10.,
    }.construct();

    // World
    let world = scenes::random_balls();

    cam.render_par(&world).save("output.png").unwrap();
}
