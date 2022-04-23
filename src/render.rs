use crate::geom::*;
use crate::object::{Object, Ray};
use crate::scenes::Environment;
use rayon::prelude::*;
use rand::prelude::*;

pub fn ray_color(r: &Ray, background: Color, world: &impl Object, depth: u32) -> Color {
    if depth == 0 {
        return BLACK;
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted = rec.material.color_emitted(rec.u, rec.v, rec.p);
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        background
    }
}

fn write_color(data: &mut Vec<u8>, pixel_color: Color, samples_per_pixel: u32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    data.push((255.999 * r) as u8);
    data.push((255.999 * g) as u8);
    data.push((255.999 * b) as u8);
}

pub fn render(environment: &Environment) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let w = environment.width();
    let h = environment.height();

    for j in (0..h).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);
        let scanline: Vec<Color> = (0..w)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = BLACK;
                for _ in 0..environment.samples_per_pixel() {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((w - 1) as f64);
                    let v = ((j as f64) + random_v) / ((h - 1) as f64);

                    let r = environment.camera.get_ray(u, v);
                    pixel_color += ray_color(
                        &r,
                        environment.background(),
                        &environment.scene,
                        environment.max_depth(),
                    );
                }
                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            write_color(&mut data, pixel_color, environment.samples_per_pixel());
        }
    }
    data
}