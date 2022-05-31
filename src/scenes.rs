use crate::bvh::*;
use crate::camera::Camera;
use crate::geom::*;
use crate::material::*;
use crate::object::*;
use crate::rect::*;
use crate::sphere::*;
use crate::texture::*;
use rand::prelude::*;
use std::sync::Arc;

pub struct RenderParams {
    pub background: Color,
    pub apsect_ratio: f32,
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl RenderParams {
    pub fn new(
        background: Color,
        apsect_ratio: f32,
        width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let height = (width as f32 / apsect_ratio) as u32;
        Self {
            background,
            apsect_ratio,
            width,
            height,
            samples_per_pixel,
            max_depth,
        }
    }
}

pub struct Environment {
    pub scene: Box<dyn Object>,
    pub camera: Camera,
    pub lights: Arc<dyn Object>,
    pub params: RenderParams,
}

impl Environment {
    pub fn new(
        scene: Box<dyn Object>,
        camera: Camera,
        lights: Arc<dyn Object>,
        params: RenderParams,
    ) -> Self {
        Self {
            scene,
            camera,
            lights,
            params,
        }
    }

    pub fn background(&self) -> Color {
        self.params.background
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.params.apsect_ratio
    }

    pub fn width(&self) -> u32 {
        self.params.width
    }

    pub fn height(&self) -> u32 {
        self.params.height
    }

    pub fn samples_per_pixel(&self) -> u32 {
        self.params.samples_per_pixel
    }

    pub fn max_depth(&self) -> u32 {
        self.params.max_depth
    }
}

pub fn cornell_box(smoke: bool) -> Environment {
    let mut objects = Objects::new(Vec::new());
    let red = lambertian(0.65, 0.05, 0.05);
    let white = lambertian(0.73, 0.73, 0.73);
    let blue = lambertian(0.12, 0.15, 0.45);
    let yellow = lambertian(0.85, 0.65, 0.15);
    let light = diffuse_light(15.0, 15.0, 15.0);
    let aluminum = metal(0.8, 0.85, 0.88, 0.0);
    let bronze = metal(0.9, 0.5, 0.5, 1.0);
    let glass = dielectric(1.5);
    objects.add(Rect::new(
        Axis::X,
        0.0,
        0.0,
        555.0,
        555.0,
        555.0,
        blue.clone(),
    ));
    objects.add(Rect::new(Axis::X, 0.0, 0.0, 555.0, 555.0, 0.0, red));
    let light_rect = Rect::new(
        Axis::Y,
        213.0 - 30.0,
        227.0 - 30.0,
        343.0 + 30.0,
        332.0 + 30.0,
        554.0,
        light.clone(),
    );
    objects.add(FlipFace::new(light_rect.clone()));
    objects.add(Rect::new(
        Axis::Y,
        0.0,
        0.0,
        555.0,
        555.0,
        0.0,
        white.clone(),
    ));
    objects.add(Rect::new(
        Axis::Y,
        0.0,
        0.0,
        555.0,
        555.0,
        555.0,
        white.clone(),
    ));
    objects.add(Rect::new(
        Axis::Z,
        0.0,
        0.0,
        555.0,
        555.0,
        555.0,
        white.clone(),
    ));
    let mut box1 = Cuboid::new(Vec3::ZERO, point3(165.0, 330.0, 165.0), aluminum.clone());
    let tr = Mat4::from_translation(vec3(250.0, 0.0, 295.0))
        * Mat4::from_rotation_y(degrees_to_radians(15.0));
    box1.add_transform(tr);
    if smoke {
        objects.add(ConstantMedium::new(box1, BLACK, 0.01));
    } else {
        objects.add(box1);
    }
    objects.add(Sphere::new(point3(190.0, 90.0, 190.0), 90.0, glass));
    objects.add(Sphere::new(
        point3(170.0, 90.0, 170.0),
        20.0,
        aluminum.clone(),
    ));
    objects.add(Sphere::new(point3(400.0, 30.0, 20.0), 30.0, bronze));
    objects.add(
        Sphere::new(point3(0.0, 0.0, 0.0), 1.0, yellow).set_transform(
            Mat4::from_translation(vec3(335.0, 35.0, 35.0)) * Mat4::from_scale(Vec3::splat(35.0)),
        ),
    );

    let camera = Camera::basic(
        point3(278.0, 278.0, -800.0),
        point3(278.0, 278.0, 0.0),
        40.0,
        1.0,
        0.0,
        10.0,
    );
    let rparams = RenderParams::new(BLACK, 1.0, 600, 25, 50);
    let mut lights = Objects::new(Vec::new());
    let light3 = Sphere::new(point3(190.0, 90.0, 190.0), 90., light);
    lights.add(light_rect);
    lights.add(light3);
    Environment::new(Box::new(objects), camera, Arc::new(lights), rparams)
}

pub fn book2_final_scene() -> Environment {
    let mut objects = Objects::new(Vec::new());
    let mut rng = thread_rng();
    let mut boxes1 = Objects::new(Vec::new());
    let ground = lambertian(0.48, 0.83, 0.53);

    const BOXES_PER_SIDE: usize = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;
            boxes1.add(Cuboid::new(
                point3(x0, y0, z0),
                point3(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    let n = boxes1.objects.len();
    objects.add(BvhNode::new(&mut boxes1, 0, n, 0.0..1.0));
    let light = diffuse_light(7.0, 7.0, 7.0);
    let light_rect = Rect::new(Axis::Y, 123.0, 147.0, 423.0, 412.0, 554.0, light);
    objects.add(FlipFace::new(light_rect.clone()));

    let center1 = point3(400.0, 400.0, 200.0);
    let center2 = center1 + vec3(30.0, 0.0, 0.0);
    let moving_sphere_material = lambertian(0.7, 0.3, 0.1);
    objects.add(Sphere::new_moving(
        center1,
        center2,
        50.0,
        moving_sphere_material,
        0.0..1.0,
    ));
    objects.add(Sphere::new(
        point3(260.0, 150.0, 45.0),
        50.0,
        dielectric(1.5),
    ));
    objects.add(Sphere::new(
        point3(0.0, 150.0, 145.0),
        50.0,
        metal(0.8, 0.8, 0.9, 1.0),
    ));

    let boundary1 = Sphere::new(point3(360.0, 150.0, 145.0), 70.0, dielectric(1.5));
    let boundary2 = Sphere::new(point3(360.0, 150.0, 145.0), 70.0, dielectric(1.5));
    objects.add(boundary1);
    objects.add(ConstantMedium::new(boundary2, color(0.2, 0.4, 0.9), 0.2));
    let boundary = Sphere::new(Vec3::ZERO, 5000.0, dielectric(1.5));
    objects.add(ConstantMedium::new(boundary, WHITE, 0.0001));

    let earth_texture =
        ImageTexture::new("/Users/jeffreyrosenbluth/Develop/ray/assets/earthmap.jpeg");
    let earth = lambertian_texture(earth_texture);
    objects.add(Sphere::new(point3(400.0, 200.0, 400.0), 100.0, earth));
    let perlin_texture = PerlinTexture::new(0.08);
    let perlin = lambertian_texture(perlin_texture);
    objects.add(Sphere::new(point3(220.0, 280.0, 300.0), 80.0, perlin));

    let mut boxes2 = Objects::new(Vec::new());
    let white = lambertian(0.73, 0.73, 0.73);
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Sphere::new(
            rand_point(&mut rng, 0.0..165.0),
            10.0,
            white.clone(),
        ));
    }

    objects.add(Translate::new(
        Rotate::new(Axis::Y, BvhNode::new(&mut boxes2, 0, ns, 0.0..1.0), 15.0),
        vec3(-100.0, 270.0, 395.0),
    ));

    let camera = Camera::new(
        point3(478.0, 278.0, -600.0),
        point3(278.0, 278.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        40.0,
        1.0,
        0.0,
        10.0,
        0.0..1.0,
    );
    let rparams = RenderParams::new(BLACK, 1.0, 800, 1000, 50);
    Environment::new(Box::new(objects), camera, Arc::new(light_rect), rparams)
}

pub fn marbles_scene() -> Environment {
    let mut rng = rand::thread_rng();
    let mut world = Objects::new(Vec::new());
    let checker = lambertian_texture(CheckeredTexture::with_color(
        vec3(0.3, 0.3, 0.3),
        vec3(0.9, 0.9, 0.9),
    ));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, checker);
    world.add(ground_sphere);
    let mut marbles = Objects::new(Vec::new());

    for a in -11..=11 {
        for b in -11..=11 {
            // don't put a marble on top of large metal sphere
            if a == 4 && b == 0 {
                continue;
            };
            let choose_mat: f32 = rng.gen();
            let center = Point3::new(
                (a as f32) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f32) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.85 {
                // Diffuse
                let albedo = rand_color(&mut rng, 0.0..1.0) * rand_color(&mut rng, 0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::solid_color(albedo));
                let _center2 = center + vec3(0.0, rng.gen_range(0.0..0.5), 0.0);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                marbles.add(sphere);
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = rand_color(&mut rng, 0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                marbles.add(sphere);
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                marbles.add(sphere);
            }
        }
    }
    let n = marbles.objects.len();
    world.add(BvhNode::new(&mut marbles, 0, n, 0.0..1.0));

    let mat1 = dielectric(1.5);
    let mat2 = lambertian(0.4, 0.2, 0.1);
    let mat3 = metal(0.7, 0.6, 0.5, 0.0);

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);

    let n = world.objects.len();
    let camera = Camera::basic(point3(13.0, 2.0, 3.0), Vec3::ZERO, 20.0, 1.5, 0.1, 10.0);
    let rparams = RenderParams::new(color(0.73, 0.73, 0.73), 1.5, 1200, 10, 50);
    Environment::new(
        Box::new(BvhNode::new(&mut world, 0, n, 0.0..1.0)),
        camera,
        Arc::new(EmptyObject {}),
        rparams,
    )
}
