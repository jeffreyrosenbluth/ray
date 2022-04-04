use camera::*;
use geom::*;
use png::*;
use rand::prelude::*;
use ray::*;
use sphere::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::rc::Rc;

mod camera;
mod geom;
mod ray;
mod sphere;

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
    data.push(255);
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
    let ref mut w = BufWriter::new(file);
    let mut encoder = Encoder::new(w, width, height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}

fn ray_color<R: Rng + ?Sized>(rng: &mut R, r: &Ray, world: &Objects, depth: u32) -> Color {
    if depth <= 0 {
        return ZERO;
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(rng, &scattered, world, depth - 1)
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = Objects::new(Vec::new());
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let sphere_ground = Sphere::new(point3(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(point3(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new(point3(-1.0, 0.0, -1.0), 0.5, material_left.clone());
    let sphere_inner = Sphere::new(point3(-1.0, 0.0, -1.0), -0.4, material_left.clone());
    let sphere_right = Sphere::new(point3(1.0, 0.0, -1.0), 0.5, material_right);

    world.add(Box::new(sphere_ground));
    world.add(Box::new(sphere_center));
    world.add(Box::new(sphere_left));
    world.add(Box::new(sphere_inner));
    world.add(Box::new(sphere_right));

    // Camera
    let lookfrom = point3(3.0, 3.0, 2.0);
    let lookat = point3(0.0, 0.0, -1.0);
    let vup = vec3(0.0 ,1.0, 0.0);
    let vfov = 20.0;
    let dist_to_focus = (lookfrom-lookat).length();
    let aperture = 2.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus
    );

    let mut data: Vec<u8> = Vec::new();
    let w = image_width;
    let h = image_height;
    let mut rng = thread_rng();
    for j in (0..h).rev() {
        for i in 0..w {
            let mut pixel_color = BLACK;
            for _ in 0..samples_per_pixel {
                let i_rand: f64 = rng.gen();
                let j_rand: f64 = rng.gen();
                let u = (i as f64 + i_rand) / (image_width as f64 - 1.0);
                let v = (j as f64 + j_rand) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&mut rng, &r, &world, max_depth);
            }
            write_color(&mut data, pixel_color, samples_per_pixel);
        }
    }
    write_png(&data, w, h, "image");
}
