use nalgebra::{vector, Point3, Vector3};
use rand::Rng;

use crate::random::rng_vec_bound;

pub struct Perlin {
    ranvec: Vec<Vector3<f64>>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Perlin {
        let ranvec = (0..point_count).map(|_| rng_vec_bound(-1., 1.)).collect();

        let perm_x = perlin_generate_perm(point_count);
        let perm_y = perlin_generate_perm(point_count);
        let perm_z = perlin_generate_perm(point_count);

        Perlin {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn turb(&self, point: Point3<f64>, depth: u32) -> f64 {
        let mut accum = 0.;
        let mut temp_point = point;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(temp_point);
            weight *= 0.5;
            temp_point *= 2.;
        }

        accum.abs()
    }

    pub fn noise(&self, point: Point3<f64>) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c: [[[Vector3<f64>; 2]; 2]; 2] = [[[vector![0., 0., 0.]; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize]
                }
            }
        }

        trilinear_interp(c, u, v, w)
    }
}

fn trilinear_interp(c: [[[Vector3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut accum = 0.;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = vector![u - i as f64, v - j as f64, w - k as f64];
                accum += (i as f64 * uu + (1. - i as f64) * (1. - uu))
                    * (j as f64 * vv + (1. - j as f64) * (1. - vv))
                    * (k as f64 * ww + (1. - k as f64) * (1. - ww))
                    * c[i][j][k].dot(&weight_v);
            }
        }
    }
    accum
}

fn perlin_generate_perm(point_count: usize) -> Vec<i32> {
    let mut p: Vec<i32> = (0..point_count).map(|i| i as i32).collect();
    for i in (1..point_count).rev() {
        let target: usize = rand::thread_rng().gen_range(0..i);
        p.swap(i, target)
    }
    p
}
