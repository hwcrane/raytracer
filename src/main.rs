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
pub mod rotation;
pub mod translate;
pub mod constant_medium;

extern crate nalgebra as na;

use bvh_node::BvhNode;
pub use na::{Point3, Vector3};

fn main() {

    let (world, cam) = scenes::final_scene(800, 10000, 40);
    let nodes = BvhNode::new(&world.objects);

    cam.render_par(&nodes).save("output.png").unwrap();
}
