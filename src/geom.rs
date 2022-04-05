use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const INFINITY: f64 = std::f64::MAX;
pub const PI: f64 = std::f64::consts::PI;
pub const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

pub const ZERO: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const ONE: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length2(self) -> f64 {
        dot(self, self)
    }

    pub fn length(self) -> f64 {
        self.length2().sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }

    pub fn map(self, f: fn(f64) -> f64) -> Self {
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

pub fn dist2(v: Vec3, w: Vec3) -> f64 {
    vec3(v.x - w.x, v.y - w.y, v.z - w.z).length2()
}

pub fn dist(v: Vec3, w: Vec3) -> f64 {
    vec3(v.x - w.x, v.y - w.y, v.z - w.z).length()
}

pub fn dot(v: Vec3, w: Vec3) -> f64 {
    v.x * w.x + v.y * w.y + v.z * w.z
}

pub fn cross(v: Vec3, w: Vec3) -> Vec3 {
    vec3(
        v.y * w.z - v.z * w.y,
        v.z * w.x - v.x * w.z,
        v.x * w.y - v.y * w.x,
    )
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, eta_ratio: f64) -> Vec3 {
    let uv = v.normalize();
    let dt = dot(uv, n);
    let discriminant = 1.0 - eta_ratio * eta_ratio * (1.0 - dt * dt);
    eta_ratio * (uv - n * dt) - n * discriminant.sqrt()
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        vec3(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        vec3(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        vec3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let z: f64 = rng.gen();
        Vec3 { x, y, z }
    }
}

pub fn rand_in_cube<R: Rng>(rng: &mut R) -> Vec3 {
    let x: f64 = rng.gen_range(-1.0..1.0);
    let y: f64 = rng.gen_range(-1.0..1.0);
    let z: f64 = rng.gen_range(-1.0..1.0);
    Vec3 { x, y, z }
}

pub fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    let mut p: Point3;
    loop {
        p = rand_in_cube(rng);
        if p.length2() >= 1.0 {
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

pub fn rand_color<R: Rng>(rng: &mut R, range: std::ops::Range<f64>) -> Color {
    let x: f64 = rng.gen_range(range.clone());
    let y: f64 = rng.gen_range(range.clone());
    let z: f64 = rng.gen_range(range);
    Color { x, y, z }
}

impl std::ops::Index<u8> for Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out or range for Vec3")
        }
    }
}
