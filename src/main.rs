pub mod core;
pub mod utility;
pub mod wrappers;
pub mod materials;
pub mod scenes;
pub mod shapes;

extern crate nalgebra as na;

pub use na::{Point3, Vector3};
use shapes::BvhNode;

fn main() {

    let (world, cam) = scenes::final_scene(800, 10000, 40);
    let nodes = BvhNode::new(&world.objects);

    cam.render_par(&nodes).save("output.png").unwrap();
}
