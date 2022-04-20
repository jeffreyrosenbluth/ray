use crate::bvh::*;
use crate::geom::*;
use crate::material::*;
use crate::object::*;
use crate::sphere::*;
use crate::texture::*;
use crate::rect::*;
use rand::prelude::*;
use std::sync::Arc;

pub fn glass_scene() -> impl Object {
    let mut world = Objects::new(Vec::new());
    let mat_ground = Arc::new(Lambertian::solid_color(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Dielectric::new(1.5));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Lambertian::solid_color(Color::new(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        point3(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    )));
    world.add(Box::new(Sphere::new(
        point3(0.0, 0.0, -1.0),
        0.5,
        mat_center,
    )));
    world.add(Box::new(Sphere::new(
        point3(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));
    world.add(Box::new(Sphere::new(
        point3(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));

    world
}

pub fn simple_light() -> impl Object {
    let perlin = Arc::new(Lambertian::new(PerlinTexture::new(2.0)));
    let mut objects = Objects::new(Vec::new());
    objects.add(Box::new(Sphere::new(
        point3(0.0, -1000.0, 0.0),
        1000.0,
        perlin.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        point3(0.0, 2.0, 0.0),
        2.0,
        perlin.clone(),
    )));
    let difflight = Arc::new(DiffuseLight::new(point3(4.0, 4.0, 4.0)));
    let difflight2 = Arc::new(DiffuseLight::new(point3(8.0, 8.0, 8.0)));
    objects.add(Box::new(Rect::new(Axis::Z, 3.0, 1.0, 5.0, 3.0, -2.0, difflight)));
    objects.add(Box::new(Sphere::new(point3(0.0, 7.0, 0.0), 1.5, difflight2)));
    objects
}

pub fn cornell_box() -> impl Object {
    let mut objects = Objects::new(Vec::new());
    let red = Arc::new(Lambertian::new(point3(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(point3(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(point3(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(point3(15.0, 15.0, 15.0)));
    objects.add(Box::new(Rect::new(Axis::X, 0.0, 0.0, 555.0, 555.0, 555.0, green)));
    objects.add(Box::new(Rect::new(Axis::X, 0.0, 0.0, 555.0, 555.0, 0.0, red)));
    objects.add(Box::new(Rect::new(Axis::Y, 213.0, 227.0, 343.0, 332.0, 554.0, light)));
    objects.add(Box::new(Rect::new(Axis::Y, 0.0, 0.0, 555.0, 555.0, 0.0, white.clone())));
    objects.add(Box::new(Rect::new(Axis::Y, 0.0, 0.0, 555.0, 555.0, 555.0, white.clone())));
    objects.add(Box::new(Rect::new(Axis::Z, 0.0, 0.0, 555.0, 555.0, 555.0, white.clone())));
    let box1 = Cuboid::new(ZERO, point3(165.0, 330.0, 165.0),white.clone());
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, vec3(250.0, 0.0, 295.0));
    objects.add(Box::new(box1));
    let box2 = Cuboid::new(ZERO, point3(165.0, 165.0, 165.0),white.clone());
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, vec3(130.0, 0.0, 65.0));
    objects.add(Box::new(box2));
    // objects.add(Box::new(ConstantMedium::new(box2, WHITE, 0.01)));
    /*objects.add(make_shared<constant_medium>(box1, 0.01, color(0,0,0)));
    objects.add(make_shared<constant_medium>(box2, 0.01, color(1,1,1))); */
    objects

}

pub fn two_perlin_spheres() -> impl Object {
    let perlin = Arc::new(Lambertian::new(PerlinTexture::new(2.0)));
    let even = vec3(0.3, 0.1, 0.1);
    let odd = vec3(0.9, 0.9, 0.9);
    let checker = Arc::new(Lambertian::new(CheckeredTexture::with_color(even, odd)));
    let mut objects = Objects::new(Vec::new());
    objects.add(Box::new(Sphere::new(
        point3(0.0, -1000.0, 0.0),
        1000.0,
        checker.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        point3(0.0, 2.0, 0.0),
        2.0,
        perlin,
    )));
    objects
}

pub fn earth() -> impl Object {
    let earth_texture = ImageTexture::new("/Users/jeffreyrosenbluth/Rust/ray/assets/earthmap.jpeg");
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    Sphere::new(point3(0.0, 0.0, 0.0), 2.0, earth_surface)
}

pub fn two_spheres() -> impl Object {
    let even = vec3(0.3, 0.1, 0.1);
    let odd = vec3(0.9, 0.9, 0.9);
    let checker = Arc::new(Lambertian::new(CheckeredTexture::with_color(even, odd)));
    let mut objects = Objects::new(Vec::new());
    objects.add(Box::new(Sphere::new(
        point3(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        point3(0.0, 10.0, 0.0),
        10.0,
        checker.clone(),
    )));
    objects
}

pub fn random_scene() -> impl Object {
    let mut rng = rand::thread_rng();
    let mut world = Objects::new(Vec::new());

    let checker = Arc::new(Lambertian::new(CheckeredTexture::with_color(
        vec3(0.3, 0.1, 0.1),
        vec3(0.9, 0.9, 0.9),
    )));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, checker);

    world.add(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            // don't put a marble on top of large metal sphere
            if a == 4 && b == 0 {
                continue;
            };
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.85 {
                // Diffuse
                let albedo = rand_color(&mut rng, 0.0..1.0) * rand_color(&mut rng, 0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::solid_color(albedo));
                let center2 = center + vec3(0.0, rng.gen_range(0.0..0.5), 0.0);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = rand_color(&mut rng, 0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::solid_color(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere3));

    let n = world.objects.len();
    BvhNode::new(&mut world, 0, n, 0.0..1.0)
}
