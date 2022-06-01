use ray::io::write_png;
use ray::render::*;
use ray::scenes::*;

fn main() {
    // let mut data: Vec<u8> = Vec::new();
    let environment = book2_final_scene();
    let data = render(&environment);
    write_png(&data, environment.width(), environment.height(), "image");
}
