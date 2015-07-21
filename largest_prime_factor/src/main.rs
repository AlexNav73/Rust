
/// 
/// Needs to find max prime divisior of geven number.
///

extern crate primes;

fn main() {

    let num: u64 = 600851475143;

    let devisors: Vec<u64> = (2..num)
        .filter(|x| num % x == 0 && primes::is_prime(*x))
        .collect();

    let max_dev: &u64 = devisors
        .iter()
        .max()
        .unwrap();

    println!("{:?}", *max_dev);

}
