use crate::geom::*;
use crate::object::{Object, Ray};
use crate::pdf::*;
use crate::scenes::Environment;
use rand::prelude::*;
use rayon::prelude::*;
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
        /*auto p0 = make_shared<hittable_pdf>(lights, rec.p);
    auto p1 = make_shared<cosine_pdf>(rec.normal);
    mixture_pdf mixed_pdf(p0, p1);

    scattered = ray(rec.p, mixed_pdf.generate(), r.time());
    pdf_val = mixed_pdf.value(scattered.direction()); */
        let emitted = rec.material.color_emitted(&rec, rec.u, rec.v, rec.p);
        let pdf0 = Arc::new(ObjectPdf::new(lights.clone(), rec.p));
        let pdf1 = Arc::new(CosinePdf::with_w(rec.normal));
        let mixture_pdf = MixturePdf::new(pdf0, pdf1);
        let scattered = Ray::new(rec.p, mixture_pdf.generate(), r.time);
        let pdf_val = mixture_pdf.value(scattered.direction);

        if let Some((attenuation, _scattered, _pdf)) = rec.material.scatter(r, &rec) {
            emitted
                + attenuation
                    * rec.material.scattering_pdf(r, &rec, &scattered)
                    * ray_color(&scattered, background, world, lights, depth - 1)
                    / pdf_val
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
