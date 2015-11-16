
extern crate clap;
extern crate primes;
extern crate num;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use num::integer::gcd;
use clap::{ App, Arg, SubCommand };

#[derive(Debug)]
struct Key {
    k: u64,
    r: u64,
}

fn set_keys(p: u64, q: u64, d: u64) -> Result<(Key, Key), &'static str> {

    match primes::is_prime(p) && primes::is_prime(q) {
        true => {
            let r = p * q;
            let phi_r = phi(r);
            let e = inverse(d, phi_r);
            Ok((Key{k: e, r: r}, Key{k: d, r: r}))
        },
        false => Err("Needs to be a prime number"),
    }
}

fn phi(mut n: u64) -> u64 {

    let mut res = n;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            res -= res / i;
        }
        i += 1;
    }

    if n > 1 {
        res -= res / n;
    }

    return res;
}

fn inverse(a: u64, n: u64) -> u64 {

    if primes::is_prime(n) {
        fast_exp(a, (n - 2), n)
    } else {
        fast_exp(a, (phi(n) - 1), n)
    }
}

fn fast_exp(a: u64, z: u64, n: u64) -> u64 { // a**z mod n

    let mut a1 = a;
    let mut z1 = z;
    let mut x = 1;

    while z1 != 0 {
        while (z1 % 2) == 0 {
            z1 /= 2;
            a1 = (a1 * a1) % n;
        }
        z1 -= 1;
        x = (x * a1) % n;
    }
    return x;
}

fn read_from_file(file_name: String) -> Result<Vec<u8>, &'static str> {

    let path = Path::new(&file_name);
    match File::open(&path) {
        Ok(file) => {
            let mut file = &file;
            let mut bytes : Vec<u8> = Vec::new();
            file.read_to_end(&mut bytes).unwrap();
            Ok(bytes)
        },
        Err(_) => Err("Can't open file."),
    }
}

fn write_to_file(file_name: String, text: Vec<u8>) {

    let path = Path::new(&file_name);
    match File::create(&path) {
        Ok(file) => {
            let mut file = &file;
            file.write(&*text.into_boxed_slice())
                .ok()
                .expect("Can't write in file.");
        },
        Err(_) => panic!("Can't open file to write."),
    }
}

fn translate_byte_seq(message: &mut Vec<u64>, key: &Key) -> Vec<u64> {

    let mut ret = Vec::new();
    for byte in message.iter() {
        ret.push(fast_exp(*byte, key.k, key.r));
    }
    return ret;
}

fn u8_to_u64(message: &mut Vec<u8>) -> Vec<u64> {

    let mut v: Vec<u64> = Vec::new();
    let mut len = message.len();
    let mut i: usize = 0;

    while len % 4 != 0 {
        message.push(32);
        len = message.len();
    }

    while i + 3 < len {
        let h1: u64 = (message[i] as u64) << 24;
        let h2: u64 = (message[i + 1] as u64) << 16;
        let h3: u64 = (message[i + 2] as u64) << 8;
        let h4: u64 = message[i + 3] as u64;
        v.push(h1 + h2 + h3 + h4);
        i += 4;
    }

    return v;
}

fn u64_to_u8(message: &mut Vec<u64>) -> Vec<u8> {
    
    let mut v: Vec<u8> = Vec::new();
    let mut i: usize = 0;

    while i < message.len() {
        v.push(((message[i] & 4278190080) >> 24) as u8);
        v.push(((message[i] & 16711680) >> 16) as u8);
        v.push(((message[i] & 65280) >> 8) as u8);
        v.push((message[i] & 255) as u8);
        i += 1;
    }

    return v;
}

fn translate_file(file_name: String, key: Key) -> Vec<u64> {

    let mut text: Vec<u64>;
    match read_from_file(file_name) {
        Ok(bytes) => {
            let mut bytes = bytes;
            let mut bytes = u8_to_u64(&mut bytes);
            text = translate_byte_seq(&mut bytes, &key);
        }
        Err(e) => panic!(e),
    };

    return text;
}

fn main() {

    let matches = App::new("chiper")
        .version("0.0.1")
        .author("AlexNav73 <fifthhorseman@mail.ru>")
        .about("Mechanism to chipher files.")
        .arg(Arg::with_name("file_in")
             .short("i")
             .long("file_in")
             .help("Input file to chipher or dechipher")
             .takes_value(true))
        .arg(Arg::with_name("file_out")
             .short("o")
             .long("file_out")
             .help("Output file to chipher or dechipher")
             .takes_value(true))
        .subcommand(SubCommand::new("keys")
                    .arg(Arg::with_name("first_prime_number")
                         .short("p")
                         .takes_value(true))
                    .arg(Arg::new("second_prime_number")
                         .short("q")
                         .takes_value(true))
                    .arg(Arg::new("third_prime_number")
                         .short("e")
                         .takes_value(true)))
        .subcommand(SubCommand::new("encode")
                    .about("Encode file with encode key.")
                    .arg(Arg::with_name("e_component")
                         .short("e")
                         .help("First key's component.")
                         .takes_value(true))
                    .arg(Arg::with_name("r_component")
                         .short("r")
                         .help("Second key's component.")
                         .takes_value(true)))
        .subcommand(SubCommand::new("decode")
                    .about("Decode file with key")
                    .arg(Arg::with_name("d_component")
                         .short("d")
                         .help("First key's component.")
                         .takes_value(true))
                    .arg(Arg::with_name("r_component")
                         .short("r")
                         .help("Second key's component.")
                         .takes_value(true)))
        .get_matches();

    let file_name_in: String = match matches.value_of("file_in") {
        Some(s) => s.to_string(), 
        None => panic!("Enter input file name."),
    };

    let file_name_out: String = match matches.value_of("file_out") {
        Some(s) => s.to_string(), 
        None => panic!("Enter output file name."),
    };
    
//    p = 59333 q = 59333 e = 59341
    if let Some(key_command) = matches.subcommand_matches("keys") {
        if key_command.is_present("first_prime_number") && 
           key_command.is_present("second_prime_number") && 
           key_command.is_present("third_prime_number") {
            let p = key_command.value_of("first_pime_number")
                .unwrap()
                .parse()
                .ok()
                .expect("Enter number to -q flag.");
            let q = key_command.value_of("second_prime_number")
                .unwrap()
                .parse()
                .ok()
                .expect("Enter number to -q flag.");
            let e = key_command.value_of("third_prime_number")
                .unwrap()
                .parse()
                .ok()
                .expect("Enter number to -e flag.");

            let (en, de) = set_keys(p, q, e).unwrap();
            println!("Encode{:?}", en);
            println!("Decode{:?}", de);
        }
    } else if let Some(en_command) = matches.subcommand_matches("encode") {
        if en_command.is_present("e_component") && 
           en_command.is_present("r_component") {
            let e = en_command.value_of("e_component")
                .unwrap()
                .parse()
                .ok()
                .expect("Enter number to -q flag.");
            let r = en_command.value_of("r_component")
                .unwrap()
                .parse()
                .ok()
                .expect("Enter number to -e flag.");

            let key: Key = Key { k: e, r: r };
            let mut text = translate_file(file_name_in, key);
            let text = u64_to_u8(&mut text); 
            write_to_file(file_name_out.to_string(), text);
        }
    } else if let Some(de_command) = matches.subcommand_matches("decode") {
        if de_command.is_present("d_component") && 
           de_command.is_present("r_component") {
            let d = de_command.value_of("d_component")
                .unwrap()
                .parse()
                .ok()
                .expect("Enter number to -d flag.");
            let r = de_command.value_of("r_component")
                .unwrap()
                .parse()
                .ok()
                .expect("Enter number to -r flag.");

            let key = Key { k: d, r: r };
            let mut text = translate_file(file_name_in, key);
            let text = u64_to_u8(&mut text); 
            write_to_file(file_name_out.to_string(), text);
        }
    }
}


// Testing area
#[cfg(test)]
mod Test {

    #[test]
    fn euler_func_test() {
        assert!(phi(2700) == 720); 
        assert!(phi(10) == 4);
        assert!(phi(8) == 4);
    }

    #[test]
    fn gcd_test() {
        // assert!(5 == gcd(5, 10));
    }

    #[test]
    fn prime_test() {
        assert!(primes::is_prime(53)); 
    }

    #[test]
    fn fast_exp_test() {
        assert!(fast_exp(3, 5, 7) == 5);
        assert!(fast_exp(5, 10, 7) == 2);
    }

    #[test]
    fn get_keys_test() {
        let (en, de) = set_keys(53, 61, 71).unwrap();
        assert!(en.k == 791);
        assert!(en.r == 3233);
        assert!(de.k == 71);
        assert!(de.r == 3233);
    }

    #[test]
    fn encode_test() {
        let e = Key{ k: 3, r: 11 };
        let d = Key{ k: 7, r: 11 };
        let mut m = fast_exp(5, 3, 11);
        m = fast_exp(m, 7, 11);
        assert!(m == 5);
    }

    #[test]
    fn chipher_test() {
        // p = 5, q = 7, n = pq = 35, phi(n) = 24, d = 11, e = inv(11, 24) = 11, M = 2
        // p = 53, q = 61, n = pq = 3233, phi(n) = 3120, d = 791, e = inv(11, 24) = 71, M = 1704 
        let p = 53;
        let q = 61;
        let n = p * q;
        let phi_n = phi(n);
        let d = 791;
        let e = inverse(d, phi_n);
        let m = 1704;

        assert!(fast_exp(m, e, n) == 3106);
        assert!(fast_exp(3106, d, n) == 1704);
    }
}
