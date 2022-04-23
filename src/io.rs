use png::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

pub fn write_ppm(data: &[u8], width: u32, height: u32, name: &'static str) {
    let path = format!(r"images/{}", name);
    let mut num = 0;
    let mut sketch = PathBuf::from(format!(r"{}_{}", path, num));
    sketch.set_extension("ppm");
    while sketch.exists() {
        num += 1;
        sketch = PathBuf::from(format!(r"{}_{}", path, num));
        sketch.set_extension("ppm");
    }
    let mut file = File::create(&sketch).unwrap();
    writeln!(file, "P3").unwrap();
    writeln!(file, "{} {}", width, height).unwrap();
    writeln!(file, "255").unwrap();
    for y in 0..height {
        for x in 0..width {
            let offset = ((y * width * 3) + x * 3) as usize;
            write!(
                &mut file,
                "{} {} {} ",
                data[offset + 0],
                data[offset + 1],
                data[offset + 2]
            )
            .unwrap()
        }
        write!(&mut file, "\n").unwrap()
    }
}

pub fn write_png(data: &[u8], width: u32, height: u32, name: &'static str) {
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