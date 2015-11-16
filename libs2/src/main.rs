
#[link(name = "dlib")]
extern "C" {
    fn mul(a: i32, b: i32) -> i32;
}

fn mul_s(a: i32, b: i32) -> i32 {
    unsafe { mul(a, b) }
}

fn main() {
    println!("Hello, world! {}", mul_s(5, 5));
}
