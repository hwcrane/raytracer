use nalgebra::{point, vector, Point3, Vector3};

use super::Camera;

pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    vfov: f64,
    lookat: Point3<f64>,
    lookfrom: Point3<f64>,
    vup: Vector3<f64>,
    defocus_angle: f64,
    focus_dist: f64,
    background: Vector3<f64>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            aspect_ratio: 1.,
            image_width: 100,
            samples_per_pixel: 1,
            max_depth: 1,
            vfov: 90.,
            lookat: point![0., 0., 0.],
            lookfrom: point![0., 0., 0.],
            vup: vector![0., 1., 0.],
            defocus_angle: 0.,
            focus_dist: 10.,
            background: vector![0., 0., 0.],
        }
    }
}

impl CameraBuilder {
    // Setter for `aspect_ratio`
    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    // Setter for `image_width`
    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    // Setter for `samples_per_pixel`
    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    // Setter for `max_depth`
    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    // Setter for `vfov`
    pub fn vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    // Setter for `lookat`
    pub fn lookat(mut self, lookat: Point3<f64>) -> Self {
        self.lookat = lookat;
        self
    }

    // Setter for `lookfrom`
    pub fn lookfrom(mut self, lookfrom: Point3<f64>) -> Self {
        self.lookfrom = lookfrom;
        self
    }

    // Setter for `vup`
    pub fn vup(mut self, vup: Vector3<f64>) -> Self {
        self.vup = vup;
        self
    }

    // Setter for `defocus_angle`
    pub fn defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    // Setter for `focus_dist`
    pub fn focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    // Setter for `background`
    pub fn background(mut self, background: Vector3<f64>) -> Self {
        self.background = background;
        self
    }
    pub fn build(self) -> Camera {
        Camera::new(
            self.aspect_ratio,
            self.image_width,
            self.samples_per_pixel,
            self.max_depth,
            self.vfov,
            self.lookat,
            self.lookfrom,
            self.vup,
            self.defocus_angle,
            self.focus_dist,
            self.background,
        )
    }
}
