
extern crate rsignal;
use rsignal::signal::*;
use rsignal::chart::*;

fn print(y: f32) {
    println!("y: {}", y);
}

fn main() {
    let mut c = Chart::new();

    c.show_spectors(print);
}
