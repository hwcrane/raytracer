pub mod core;
pub mod utility;
pub mod wrappers;
pub mod materials;
pub mod scenes;
pub mod shapes;
mod gui;

extern crate nalgebra as na;

use std::sync::Arc;

pub use na::{Point3, Vector3};
use shapes::BvhNode;

fn main() {
    // let (world, cam) = scenes::final_scene(800, 1000, 40);
    let (world, cam) = scenes::cornel_box();
    let nodes = BvhNode::new(&world.objects);
    
    let reciever = Arc::new(cam).render_to_channel(Arc::new(nodes));

    gui::main(reciever, 600, 600);

}
