use rand::Rng;
pub use glam::*;

pub const INFINITY: f32 = std::f32::MAX;
pub const PI: f32 = std::f32::consts::PI;

pub type Point3 = Vec3;
pub type Color = Vec3;

pub const BLACK: Color = Vec3::ZERO;
pub const WHITE: Color = Vec3::ONE;


pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn point3(x: f32, y: f32, z: f32) -> Point3 {
    Point3::new(x, y, z)
}

pub fn color(r: f32, g: f32, b: f32) -> Color {
    point3(r, g, b)
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn transform(&self, mat: Mat4) -> Self {
        let mut r = self.clone();
        r.origin = mat.transform_point3(r.origin);
        r.direction = mat.transform_vector3(r.direction);
        r
    }
}

pub fn dot(v: Vec3, w: Vec3) -> f32 {
    v.dot(w)
}

pub fn cross(v: Vec3, w: Vec3) -> Vec3 {
    v.cross(w)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, eta_ratio: f32) -> Vec3 {
    let uv = v.normalize();
    let dt = dot(uv, n);
    let discriminant = 1.0 - eta_ratio * eta_ratio * (1.0 - dt * dt);
    eta_ratio * (uv - n * dt) - n * discriminant.sqrt()
}

pub fn rand_in_cube<R: Rng>(rng: &mut R) -> Vec3 {
    let x: f32 = rng.gen_range(-1.0..1.0);
    let y: f32 = rng.gen_range(-1.0..1.0);
    let z: f32 = rng.gen_range(-1.0..1.0);
    Vec3::new(x, y, z)
}

pub fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    let mut p: Point3;
    loop {
        p = rand_in_cube(rng);
        if p.length_squared() >= 1.0 {
            continue;
        }
        break;
    }
    p
}

pub fn random_unit_vector<R: Rng>(rng: &mut R) -> Vec3 {
    random_in_unit_sphere(rng).normalize()
}

pub fn random_in_unit_disk<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length() < 1.0 {
            return p;
        }
    }
}

pub fn random_cosine_direction<R: Rng>(rng: &mut R) -> Vec3 {
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    vec3(x, y, z)
}

pub fn rand_color<R: Rng>(rng: &mut R, range: std::ops::Range<f32>) -> Color {
    let x: f32 = rng.gen_range(range.clone());
    let y: f32 = rng.gen_range(range.clone());
    let z: f32 = rng.gen_range(range);
    Color::new(x, y, z)
}

pub fn rand_point<R: Rng>(rng: &mut R, range: std::ops::Range<f32>) -> Color {
    let x: f32 = rng.gen_range(range.clone());
    let y: f32 = rng.gen_range(range.clone());
    let z: f32 = rng.gen_range(range);
    Point3::new(x, y, z)
}

// impl std::ops::Index<u8> for Vec3 {
//     type Output = f32;

//     fn index(&self, index: u8) -> &Self::Output {
//         match index {
//             0 => &self.x,
//             1 => &self.y,
//             2 => &self.z,
//             _ => panic!("Index out or range for Vec3"),
//         }
//     }
// }

// impl std::ops::IndexMut<u8> for Vec3 {
//     fn index_mut(&mut self, index: u8) -> &mut Self::Output {
//         match index {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             2 => &mut self.z,
//             _ => panic!("Index out or range for Vec3"),
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const fn order(self) -> (Axis, Axis, Axis) {
        match self {
            Axis::X => (Axis::Y, Axis::Z, Axis::X),
            Axis::Y => (Axis::X, Axis::Z, Axis::Y),
            Axis::Z => (Axis::X, Axis::Y, Axis::Z),
        }
    }
}

impl std::ops::Index<Axis> for Vec3 {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl std::ops::IndexMut<Axis> for Vec3 {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Onb {
    pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Self {
        Self { u, v, w }
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        a.x * self.u + a.y * self.v + a.z * self.w
    }

    pub fn build_from_w(n: Vec3) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 {
            vec3(0.0, 1.0, 0.0)
        } else {
            vec3(1.0, 0.0, 0.0)
        };
        let v = cross(w, a).normalize();
        let u = cross(w, v);
        Self { u, v, w }
    }
}