
#[no_mangle]
pub extern fn mul(a: i32, b: i32) -> i32 {
    println!("{} * {} = {}", a, b, a * b);
    a * b
}
