use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const INFINITY: f64 = std::f64::MAX;
pub const PI: f64 = std::f64::consts::PI;
pub const BLACK: Color = Color {x: 0.0, y: 0.0, z: 0.0};
pub const WHITE: Color = Color {x: 1.0, y: 1.0, z: 1.0};
pub const ZERO: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
pub const ONE: Vec3 = Vec3 {x: 1.0, y: 1.0, z: 1.0};

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

    pub fn scale(self, k: f64) -> Self {
        self * k
    }

    pub fn lerp(self, other: Self, t: f64) -> Self {
        let x = self.x * (1.0 - t) + t * other.x;
        let y = self.y * (1.0 - t) + t * other.y;
        let z = self.z * (1.0 - t) + t * other.z;
        Vec3 { x, y, z }
    }

    pub fn length2(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length2().sqrt()
    }

    pub fn dist2(self, other: Self) -> f64 {
        vec3(self.x - other.x, self.y - other.y, self.z - other.z).length2()
    }

    pub fn dist(self, other: Self) -> f64 {
        vec3(self.x - other.x, self.y - other.y, self.z - other.z).length()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        vec3(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
        let uv = self.normalize();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
        } else {
            None
        }
    }
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
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
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
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0..1.0);
        let z: f64 = rng.gen_range(-1.0..1.0);
        Vec3 { x, y, z }
    }
}

pub fn random_in_unit_sphere<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
    let mut p: Point3;
    loop {
        p = rng.gen();
        if p.length2() >= 1.0 {
            continue;
        }
        break;
    }
    p
}

pub fn random_unit_vector<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
    random_in_unit_sphere(rng).normalize()
}
