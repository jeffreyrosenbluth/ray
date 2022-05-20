use crate::geom::*;
use crate::material::Reflection;
use crate::object::{Object, Ray, EmptyObject};
use crate::pdf::*;
use crate::scenes::Environment;
use rand::rngs::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};
use rayon::prelude::*;
use std::sync::Arc;

pub fn ray_color(
    rng: &mut SmallRng,
    r: &Ray,
    background: Color,
    world: &impl Object,
    lights: Arc<dyn Object>,
    depth: u32,
) -> Color {
    // let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
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
                    let scattered = Ray::new(rec.p, mixture_pdf.generate(rng), r.time);
                    let pdf_val = mixture_pdf.value(scattered.direction);
                    emitted
                        + scatter_rec.attenuation
                            * rec.material.scattering_pdf(r, &rec, &scattered)
                            * ray_color(rng, &scattered, background, world, lights, depth - 1)
                            / pdf_val
                }
                Reflection::Specular(ray) => {
                    scatter_rec.attenuation
                        * ray_color(rng, &ray, background, world, lights, depth - 1)
                }
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
                let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
                let n = (environment.samples_per_pixel() as f32).sqrt() as u32;
                for s in 0..n {
                    for t in 0..n {
                        let u = ((i as Float) + (s as f32 + rng.gen::<Float>()) / n as f32)
                            / ((w - 1) as Float);
                        let v = ((j as Float) + (t as f32 + rng.gen::<Float>()) / n as f32)
                            / ((h - 1) as Float);
                        let r = environment.camera.get_ray(u, v);
                        let mut rc = ray_color(
                            &mut rng,
                            &r,
                            environment.background(),
                            &environment.scene,
                            environment.lights.clone(),
                            environment.max_depth(),
                        );
                        if rc.x.is_nan() {
                            rc.x = 0.0
                        };
                        if rc.y.is_nan() {
                            rc.y = 0.0
                        };
                        if rc.z.is_nan() {
                            rc.z = 0.0
                        };
                        pixel_color += rc;
                    }
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