use crate::geom::*;
use image::*;
use noise::*;
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn value(&self, u: Float, v: Float, p: Point3) -> Color;
}

impl Texture for Color {
    fn value(&self, _u: Float, _v: Float, _p: Point3) -> Color {
        *self
    }
}

#[derive(Clone)]
pub struct CheckeredTexture<T, U>
where
    T: Texture,
    U: Texture,
{
    pub even: Arc<T>,
    pub odd: Arc<U>,
}

impl<T, U> CheckeredTexture<T, U>
where
    T: Texture,
    U: Texture,
{
    pub fn new(even: Arc<T>, odd: Arc<U>) -> Self {
        Self { even, odd }
    }
}

impl CheckeredTexture<Color, Color> {
    pub fn with_color(even: Color, odd: Color) -> Self {
        let even = Arc::new(even);
        let odd = Arc::new(odd);
        Self { even, odd }
    }
}

impl<T, U> Texture for CheckeredTexture<T, U>
where
    T: Texture,
    U: Texture,
{
    fn value(&self, u: Float, v: Float, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

#[derive(Clone)]
pub struct PerlinTexture {
    pub scale: Float,
    pub noise: Fbm<Perlin>,
}

impl PerlinTexture {
    pub fn new(scale: Float) -> Self {
        let noise = Fbm::<Perlin>::new(0).set_octaves(7);
        Self { scale, noise }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, _u: Float, _v: Float, p: Point3) -> Color {
        let p = p * self.scale;
        ONE * 0.5
            * (1.0
                + ((p.z * self.scale)
                    + 5.0 * (self.noise.get([p.x as f64, p.y as f64, p.z as f64]).abs() as Float))
                    .sin())
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl ImageTexture {
    pub fn new(path: &'static str) -> Self {
        let img = open(path).unwrap();
        let rgb8 = img.to_rgb8();
        let data = rgb8.to_vec();
        let width = rgb8.width() as usize;
        let height = rgb8.height() as usize;
        Self {
            data,
            width,
            height,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: Float, v: Float, _p: Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        let i = ((u * self.width as Float) as usize).min(self.width - 1);
        let j = ((v * self.height as Float) as usize).min(self.height - 1);
        let scale = 1.0 / 255.0;
        let k = j * 3 * self.width + i * 3;
        let r = self.data[k] as Float * scale;
        let g = self.data[k + 1] as Float * scale;
        let b = self.data[k + 2] as Float * scale;
        color(r, g, b)
    }
}
