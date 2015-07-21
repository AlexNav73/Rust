
mod module;

#[link(name = "lib")]
extern {
    fn mul(a: i32, b: i32) -> i32;
}

fn mul_s(a: i32, b: i32) -> i32 {
    unsafe { mul(a, b) }
}

fn main() {
    use module::*;

    println!("{}", add(1, 2));
    println!("{}", mul_s(2, 2));
}
