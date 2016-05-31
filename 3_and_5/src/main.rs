
fn main() {
    let vec: u32 = (1..10)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .collect::<Vec<u32>>()
        .iter()
        .fold(0, |sum, &x| sum + x);
    println!("{:?}", vec);
}
