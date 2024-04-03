use std::sync::Arc;

use nalgebra::{point, vector, Vector3};
use rand::{random, Rng};

use crate::{
    core::{Camera, HittableList},
    materials::{Checker, ImageTexture, Material, NoiseTexture, SolidColour},
    shapes::{make_box, BvhNode, Quad, Sphere},
    utility::random::rng_vec_bound,
    wrappers::{ConstantMedium, RotateY, Translate},
};

pub type Scene = fn() -> (HittableList, Camera);

pub fn final_scene(
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> (HittableList, Camera) {
    let cam = Camera::builder()
        .aspect_ratio(1.)
        .image_width(image_width)
        .samples_per_pixel(samples_per_pixel)
        .max_depth(max_depth)
        .vfov(40.)
        .lookfrom(point![478., 278., -600.])
        .lookat(point![278., 278., 0.])
        .vup(vector![0., 1., 0.])
        .defocus_angle(0.)
        .focus_dist(10.)
        .background(vector![0., 0., 0.])
        .build();

    let mut boxes1 = HittableList::new();
    let ground = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![0.48, 0.83, 0.53])),
    };

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let z0 = -1000. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = rand::thread_rng().gen_range(1..101) as f64;
            let z1 = z0 + w;
            boxes1.add(Box::new(make_box(
                point![x0, y0, z0],
                point![x1, y1, z1],
                &ground,
            )))
        }
    }

    let mut world = HittableList::new();
    world.add(Box::new(BvhNode::new(&boxes1.objects)));

    let light = Material::DiffuseLight {
        emit: Arc::new(SolidColour::new(vector![7., 7., 7.])),
    };
    world.add(Box::new(Quad::new(
        point![123., 554., 147.],
        vector![300., 0., 0.],
        vector![0., 0., 265.],
        &light,
    )));

    let center1 = point!(400., 400., 200.);
    let center2 = center1 + vector!(30., 0., 0.);
    let sphere_mat = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![0.7, 0.3, 0.1])),
    };
    world.add(Box::new(Sphere::new_moving(
        center1,
        center2,
        50.,
        &sphere_mat,
    )));

    world.add(Box::new(Sphere::new(
        point![260., 150., 145.],
        50.,
        &Material::Dielectric { ir: 1.5 },
    )));
    world.add(Box::new(Sphere::new(
        point![0., 150., 145.],
        50.,
        &Material::Metal {
            albedo: vector![0.8, 0.8, 0.9],
            fuzz: 1.,
        },
    )));
    let boundary = Sphere::new(point![0., 0., 0.], 5000., &Material::Dielectric { ir: 1.5 });
    world.add(Box::new(ConstantMedium::new(
        Arc::new(boundary),
        0.0001,
        Arc::new(SolidColour::new(vector![1., 1., 1.])),
    )));

    let emat = Material::Lambertian {
        albedo: Arc::new(ImageTexture::new("earthmap.jpg").unwrap()),
    };
    world.add(Box::new(Sphere::new(point![400., 200., 400.], 100., &emat)));
    let pertext = NoiseTexture::new(0.1);
    world.add(Box::new(Sphere::new(
        point![220., 280., 300.],
        80.,
        &Material::Lambertian {
            albedo: Arc::new(pertext),
        },
    )));

    let mut boxes2 = HittableList::new();
    let white = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![0.73, 0.73, 0.73])),
    };
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Box::new(Sphere::new(
            rng_vec_bound(0., 165.).into(),
            10.,
            &white,
        )));
    }

    world.add(Box::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::new(&boxes2.objects)), 15.)),
        vector![-100., 270., 395.],
    )));
    (world, cam)
}
pub fn cornel_smoke() -> (HittableList, Camera) {
    let cam = Camera::builder()
        .aspect_ratio(1.)
        .image_width(600)
        .samples_per_pixel(200)
        .max_depth(50)
        .vfov(40.)
        .lookfrom(point![278., 278., -800.])
        .lookat(point![278., 279., 0.])
        .vup(vector![0., 1., 0.])
        .defocus_angle(0.)
        .focus_dist(10.)
        .background(vector![0., 0., 0.])
        .build();
    let mut world = HittableList::new();

    let red = Material::Lambertian {
        albedo: Arc::new(SolidColour::from_rgb(0.65, 0.05, 0.05)),
    };
    let white = Material::Lambertian {
        albedo: Arc::new(SolidColour::from_rgb(0.73, 0.73, 0.73)),
    };
    let green = Material::Lambertian {
        albedo: Arc::new(SolidColour::from_rgb(0.12, 0.45, 0.15)),
    };
    let light = Material::DiffuseLight {
        emit: Arc::new(SolidColour::from_rgb(15., 15., 15.)),
    };

    world.add(Box::new(Quad::new(
        point![555., 0., 0.],
        vector![0., 555., 0.],
        vector![0., 0., 555.],
        &green,
    )));
    world.add(Box::new(Quad::new(
        point![0., 0., 0.],
        vector![0., 555., 0.],
        vector![0., 0., 555.],
        &red,
    )));
    world.add(Box::new(Quad::new(
        point![343., 554., 332.],
        vector![-130., 0., 0.],
        vector![0., 0., -105.],
        &light,
    )));
    world.add(Box::new(Quad::new(
        point![0., 0., 0.],
        vector![555., 0., 0.],
        vector![0., 0., 555.],
        &white,
    )));
    world.add(Box::new(Quad::new(
        point![555., 555., 555.],
        vector![-555., 0., 0.],
        vector![0., 0., -555.],
        &white,
    )));
    world.add(Box::new(Quad::new(
        point![0., 0., 555.],
        vector![555., 0., 0.],
        vector![0., 555., 0.],
        &white,
    )));

    let box1 = make_box(point![0., 0., 0.], point![165., 330., 165.], &white);
    let box1 = RotateY::new(Arc::new(box1), 15.);
    let box1 = Translate::new(Arc::new(box1), vector![265., 0., 295.]);
    world.add(Box::new(ConstantMedium::new(
        Arc::new(box1),
        0.01,
        Arc::new(SolidColour::new(vector![0., 0., 0.])),
    )));

    let box2 = make_box(point![0., 0., 0.], point![165., 166., 165.], &white);
    let box2 = RotateY::new(Arc::new(box2), -18.);
    let box2 = Translate::new(Arc::new(box2), vector![130., 0., 65.]);
    world.add(Box::new(ConstantMedium::new(
        Arc::new(box2),
        0.01,
        Arc::new(SolidColour::new(vector![1., 1., 1.])),
    )));

    (world, cam)
}

pub fn cornel_box() -> (HittableList, Camera) {
    let cam = Camera::builder()
        .aspect_ratio(1.)
        .image_width(600)
        .samples_per_pixel(200)
        .max_depth(50)
        .vfov(40.)
        .lookfrom(point![278., 278., -800.])
        .lookat(point![278., 279., 0.])
        .vup(vector![0., 1., 0.])
        .defocus_angle(0.)
        .focus_dist(10.)
        .background(vector![0., 0., 0.])
        .build();

    let mut world = HittableList::new();

    let red = Material::Lambertian {
        albedo: Arc::new(SolidColour::from_rgb(0.65, 0.05, 0.05)),
    };
    let white = Material::Lambertian {
        albedo: Arc::new(SolidColour::from_rgb(0.73, 0.73, 0.73)),
    };
    let green = Material::Lambertian {
        albedo: Arc::new(SolidColour::from_rgb(0.12, 0.45, 0.15)),
    };
    let light = Material::DiffuseLight {
        emit: Arc::new(SolidColour::from_rgb(15., 15., 15.)),
    };

    world.add(Box::new(Quad::new(
        point![555., 0., 0.],
        vector![0., 555., 0.],
        vector![0., 0., 555.],
        &green,
    )));
    world.add(Box::new(Quad::new(
        point![0., 0., 0.],
        vector![0., 555., 0.],
        vector![0., 0., 555.],
        &red,
    )));
    world.add(Box::new(Quad::new(
        point![343., 554., 332.],
        vector![-130., 0., 0.],
        vector![0., 0., -105.],
        &light,
    )));
    world.add(Box::new(Quad::new(
        point![0., 0., 0.],
        vector![555., 0., 0.],
        vector![0., 0., 555.],
        &white,
    )));
    world.add(Box::new(Quad::new(
        point![555., 555., 555.],
        vector![-555., 0., 0.],
        vector![0., 0., -555.],
        &white,
    )));
    world.add(Box::new(Quad::new(
        point![0., 0., 555.],
        vector![555., 0., 0.],
        vector![0., 555., 0.],
        &white,
    )));

    let box1 = make_box(point![0., 0., 0.], point![165., 330., 165.], &white);
    let box1 = RotateY::new(Arc::new(box1), 15.);
    let box1 = Translate::new(Arc::new(box1), vector![265., 0., 295.]);
    world.add(Box::new(box1));

    let box2 = make_box(point![0., 0., 0.], point![165., 165., 165.], &white);
    let box2 = RotateY::new(Arc::new(box2), -18.);
    let box2 = Translate::new(Arc::new(box2), vector![130., 0., 65.]);
    world.add(Box::new(box2));

    (world, cam)
}

pub fn simple_light() -> (HittableList, Camera) {
    let cam = Camera::builder()
        .aspect_ratio(16. / 9.)
        .image_width(400)
        .samples_per_pixel(500)
        .max_depth(500)
        .vfov(20.)
        .lookfrom(point![26., 3., 6.])
        .lookat(point![0., 2., 0.])
        .vup(vector![0., 1., 0.])
        .defocus_angle(0.)
        .focus_dist(10.)
        .background(vector![0., 0., 0.])
        .build();

    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.));
    let mat = Material::Lambertian {
        albedo: pertext.clone(),
    };
    world.add(Box::new(Sphere::new(point![0., -1000., 0.], 1000., &mat)));
    world.add(Box::new(Sphere::new(point![0., 2., 0.], 2., &mat)));

    let difflight = Material::DiffuseLight {
        emit: Arc::new(SolidColour::new(vector![4., 4., 4.])),
    };
    world.add(Box::new(Sphere::new(point![0., 7., 0.], 2., &difflight)));
    world.add(Box::new(Quad::new(
        point![3., 1., -2.],
        vector![2., 0., 0.],
        vector![0., 2., 0.],
        &difflight,
    )));

    (world, cam)
}

pub fn quads() -> (HittableList, Camera) {
    let cam = Camera::builder()
        .aspect_ratio(1.)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(80.)
        .lookfrom(point![0., 0., 9.])
        .lookat(point![0., 0., 0.])
        .vup(vector![0., 1., 0.])
        .defocus_angle(0.)
        .focus_dist(10.)
        .background(vector![0.7, 0.8, 1.])
        .build();

    let mut world = HittableList::new();

    let left_red = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![1., 0.2, 0.2])),
    };
    let back_green = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![0.2, 1.0, 0.2])),
    };
    let right_blue = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![0.2, 0.2, 1.])),
    };
    let upper_orange = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![1., 0.5, 0.0])),
    };
    let lower_teal = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![0.2, 0.8, 0.8])),
    };

    world.add(Box::new(Quad::new(
        point![-3., -2., 5.],
        vector![0., 0., -4.],
        vector![0., 4., 0.],
        &left_red,
    )));
    world.add(Box::new(Quad::new(
        point![-2., -2., 0.],
        vector![4., 0., 0.],
        vector![0., 4., 0.],
        &back_green,
    )));
    world.add(Box::new(Quad::new(
        point![3., -2., 1.],
        vector![0., 0., 4.],
        vector![0., 4., 0.],
        &right_blue,
    )));
    world.add(Box::new(Quad::new(
        point![-2., 3., 1.],
        vector![4., 0., 0.],
        vector![0., 0., 4.],
        &upper_orange,
    )));
    world.add(Box::new(Quad::new(
        point![-2., -3., 5.],
        vector![4., 0., 0.],
        vector![0., 0., -4.],
        &lower_teal,
    )));
    (world, cam)
}

pub fn two_perlin_spheres() -> (HittableList, Camera) {
    let cam = Camera::builder()
        .aspect_ratio( 16. / 9.)
        .image_width( 400)
        .samples_per_pixel( 100)
        .max_depth( 50)
        .vfov( 20.)
        .lookfrom( point![13., 2., 3.])
        .lookat( point![0., 0., 0.])
        .vup( vector![0., 1., 0.])
        .defocus_angle( 0.)
        .focus_dist( 10.)
        .background( vector![0.7, 0.8, 1.])
    .build();

    let mut world = HittableList::new();
    let perlin_texture = Arc::new(NoiseTexture::new(4.));
    let perlin_material = Material::Lambertian {
        albedo: perlin_texture,
    };
    world.add(Box::new(Sphere::new(
        point![0., -1000., 0.],
        1000.,
        &perlin_material,
    )));
    world.add(Box::new(Sphere::new(
        point![0., 2., 0.],
        2.,
        &perlin_material,
    )));

    (world, cam)
}

pub fn earth() -> (HittableList, Camera) {
    let cam = Camera::builder() 
        .aspect_ratio( 16. / 9.)
        .image_width( 400)
        .samples_per_pixel( 100)
        .max_depth( 50)
        .vfov( 20.)
        .lookfrom( point![13., 2., 3.])
        .lookat( point![0., 0., 0.])
        .vup( vector![0., 1., 0.])
        .defocus_angle( 0.)
        .focus_dist( 10.)
        .background( vector![0.7, 0.8, 1.])
    .build();

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg").expect("Image failed to load"));
    let earth_surface = Material::Lambertian {
        albedo: earth_texture,
    };
    let globe = Sphere::new(point![0., 0., 0.], 2., &earth_surface);
    let mut universe = HittableList::new();
    universe.add(Box::new(globe));

    (universe, cam)
}

pub fn two_spheres() -> (HittableList, Camera) {
    let cam = Camera::builder()
        .aspect_ratio( 16. / 9.)
        .image_width( 400)
        .samples_per_pixel( 100)
        .max_depth( 50)
        .vfov( 20.)
        .lookfrom( point![13., 2., 3.])
        .lookat( point![0., 0., 0.])
        .vup( vector![0., 1., 0.])
        .defocus_angle( 0.)
        .focus_dist( 10.)
        .background( vector![0.7, 0.8, 1.])
    .build();

    let mut world = HittableList::new();
    let checker = Arc::new(Checker::from_colours(
        0.32,
        vector![0.2, 0.3, 0.1],
        vector![0.9, 0.9, 0.9],
    ));

    world.add(Box::new(Sphere::new(
        point![0., -10., 0.],
        10.,
        &Material::Lambertian {
            albedo: checker.clone(),
        },
    )));
    world.add(Box::new(Sphere::new(
        point![0., 10., 0.],
        10.,
        &Material::Lambertian {
            albedo: checker.clone(),
        },
    )));
    (world, cam)
}

/// Generates the scene from the end of the Ray Tracing in One Weekend book
pub fn random_balls() -> (HittableList, Camera) {
    let cam = Camera::builder()
        .aspect_ratio( 16. / 9.)
        .image_width( 400)
        .samples_per_pixel( 100)
        .max_depth( 50)
        .vfov( 20.)
        .lookfrom( point![13., 2., 3.])
        .lookat( point![0., 0., 0.])
        .vup( vector![0., 1., 0.])
        .defocus_angle( 10.)
        .focus_dist( 10.)
        .background( vector![0.7, 0.8, 1.])
    .build();

    let mut world = HittableList::new();
    let ground_material = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![0.5, 0.5, 0.5])),
    };
    world.add(Box::new(Sphere::new(
        point![0., -1000., 0.],
        1000.,
        &ground_material,
    )));

    for a in (-11)..11 {
        for b in (-11)..11 {
            let chose_mat: f64 = random();
            let center = point![
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 as f64 * random::<f64>()
            ];

            if (center - point![4., 0.2, 0.]).norm() > 0.9 {
                if chose_mat < 0.8 {
                    let albedo: Vector3<f64> = vector![
                        random::<f64>() * random::<f64>(),
                        random::<f64>() * random::<f64>(),
                        random::<f64>() * random::<f64>()
                    ];
                    let mat = Material::Lambertian {
                        albedo: Arc::new(SolidColour::new(albedo)),
                    };
                    let center2 = center + vector![0., random::<f64>() / 2., 0.];
                    world.add(Box::new(Sphere::new_moving(center, center2, 0.2, &mat)));
                } else if chose_mat < 0.95 {
                    let albedo = rng_vec_bound(0.5, 1.);
                    let fuzz = rand::thread_rng().gen_range((0.)..0.5);
                    let mat = Material::Metal { albedo, fuzz };
                    world.add(Box::new(Sphere::new(center, 0.2, &mat)));
                } else {
                    let mat = Material::Dielectric { ir: 1.5 };
                    world.add(Box::new(Sphere::new(center, 0.2, &mat)));
                };
            }
        }
    }

    let mat1 = Material::Dielectric { ir: 1.5 };
    let mat2 = Material::Lambertian {
        albedo: Arc::new(SolidColour::new(vector![0.4, 0.2, 0.1])),
    };
    let mat3 = Material::Metal {
        albedo: vector![0.7, 0.6, 0.6],
        fuzz: 0.0,
    };

    world.add(Box::new(Sphere::new(point![0., 1., 0.], 1., &mat1)));
    world.add(Box::new(Sphere::new(point![-4., 1., 0.], 1., &mat2)));
    world.add(Box::new(Sphere::new(point![4., 1., 0.], 1., &mat3)));

    (world, cam)
}
