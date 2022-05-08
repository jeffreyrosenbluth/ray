use crate::geom::*;
use crate::material::Reflection;
use crate::object::{Object, Ray};
use crate::pdf::*;
use crate::scenes::Environment;
use noise::ScaleBias;
use rand::prelude::*;
use rayon::prelude::*;
use std::cell::Ref;
use std::sync::Arc;

pub fn ray_color(
    r: &Ray,
    background: Color,
    world: &impl Object,
    lights: Arc<dyn Object>,
    depth: u32,
) -> Color {
    if depth == 0 {
        return BLACK;
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted = rec.material.color_emitted(&rec, rec.u, rec.v, rec.p);
        if let Some(scatter_rec) = rec.material.scatter(r, &rec) {
            match scatter_rec.reflection {
                Reflection::Scatter(pdf1) => {
                    let pdf0 = Arc::new(ObjectPdf::new(lights.clone(), rec.p));
                    let mixture_pdf = MixturePdf::new(pdf0, pdf1);
                    let scattered = Ray::new(rec.p, mixture_pdf.generate(), r.time);
                    let pdf_val = mixture_pdf.value(scattered.direction);
                    emitted
                        + scatter_rec.attenuation
                            * rec.material.scattering_pdf(r, &rec, &scattered)
                            * ray_color(&scattered, background, world, lights, depth - 1)
                            / pdf_val
                }
                Reflection::Specular(ray) => scatter_rec.attenuation
                * ray_color(&ray, background, world, lights, depth-1)
            }
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

    if r.is_nan() {r = 0.0};
    if g.is_nan() {g = 0.0};
    if b.is_nan() {b = 0.0};

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as Float;
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
                    let random_u: Float = rng.gen();
                    let random_v: Float = rng.gen();

                    let u = ((i as Float) + random_u) / ((w - 1) as Float);
                    let v = ((j as Float) + random_v) / ((h - 1) as Float);

                    let r = environment.camera.get_ray(u, v);
                    pixel_color += ray_color(
                        &r,
                        environment.background(),
                        &environment.scene,
                        environment.lights.clone(),
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
