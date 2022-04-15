use png::*;
use rand::prelude::*;
use ray::camera::*;
use ray::geom::*;
use ray::object::*;
use ray:: scenes::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

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

fn write_png(data: &[u8], width: u32, height: u32, name: &'static str) {
    let path = format!(r"images/{}", name);
    let mut num = 0;
    let mut sketch = PathBuf::from(format!(r"{}_{}", path, num));
    sketch.set_extension("png");
    while sketch.exists() {
        num += 1;
        sketch = PathBuf::from(format!(r"{}_{}", path, num));
        sketch.set_extension("png");
    }
    let file = File::create(&sketch).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = Encoder::new(w, width, height);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}

fn ray_color(r: &Ray, world: &impl Object, depth: u32) -> Color {
    if depth == 0 {
        return BLACK;
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            BLACK
        }
    } else {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * WHITE + t * Color::new(0.5, 0.7, 1.0)
    }
}



fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World
    let world = earth();

    // Camera
    let cam = Camera::new(
        point3(13.0, 2.0, 3.0),
        ZERO,
        vec3(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
        0.0..1.0,
    );

    let mut data: Vec<u8> = Vec::new();
    let w = IMAGE_WIDTH;
    let h = IMAGE_HEIGHT;

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);

        let scanline: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = BLACK;
                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }
                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            write_color(&mut data, pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    write_png(&data, w, h, "image");
}
