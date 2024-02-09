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
pub mod aabb;
pub mod bvh_node;
pub mod textures;
pub mod quad;

extern crate nalgebra as na;

use bvh_node::BvhNode;
pub use na::{Point3, Vector3};

fn main() {

    let (world, cam) = scenes::cornel_box();
    let nodes = BvhNode::new(&world.objects);

    cam.render_par(&nodes).save("output.png").unwrap();
}
