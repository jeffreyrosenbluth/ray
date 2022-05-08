use ray::render::*;
use ray::io::write_png;
use ray::scenes::*;

fn main() {
    // let mut data: Vec<u8> = Vec::new();
    let environment = cornell_box(false);
    let data = render(&environment);
    write_png(&data, environment.width(), environment.height(), "image");
}
