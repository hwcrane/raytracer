mod texture;
mod solid;
mod image;
mod checker;
mod perlin;
mod noise_texture;

pub use texture::Texture;
pub use solid::SolidColour;
pub use image::ImageTexture;
pub use checker::Checker;
pub use noise_texture::NoiseTexture;

use perlin::Perlin;
