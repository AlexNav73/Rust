
extern crate libc;

use std::collections::HashMap;
use std::io::{ Read, BufReader };
use std::fs::File;

mod abi;
mod help;

pub use abi::*;
use help::{parse_text, dedup_by};

pub struct WordCounter {
    word_count: usize,
    words_entry_map: HashMap<String, u32>
}

impl Drop for WordCounter {
    fn drop(&mut self) {
        println!("Object has been dropped!");
    }
}

macro_rules! try_ext {
    ($e:expr, $mess:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                println!("{}: {}", $mess, e);
                panic!(e);
            }
        }
    }
}

impl WordCounter {

    pub fn new(path: &str) -> WordCounter {
        let mut reader = BufReader::new(
            try_ext!(File::open(path), "File open paniced"));

        let mut text = String::new();

        try_ext!(reader.read_to_string(&mut text), "Read to string paniced");

        let (count, map) = parse_text(text);

        WordCounter {
            word_count: count,
            words_entry_map: map
        }
    }


    ///
    /// Calculates probability of each word in text
    ///
    /// Converts HashMap to Vec<_> to sort by probability of words
    /// In Vec<_> index represents rang of word in [Zippha theorem]
    ///
    pub fn count_probability<'a>(&'a mut self) -> Vec<(&str, f64)> {
        // Pi = Ni / N
        let mut v = self.words_entry_map.iter()
            .map(|(k, v)| (&**k, *v as f64 / self.word_count as f64))
            .collect::<Vec<(&str, f64)>>();

        // Sort to make words with max Pi have max rang
        v.sort_by(|&(_, v1), &(_, v2)|
            v1.partial_cmp(&v2).unwrap().reverse() // FIXME: Use unwrap() - may fail
        );

        dedup_by(&mut v, |&(_, v1), &(_, v2)| v1 == v2);

        v
    }

    ///
    /// Creates vector where:
    ///     `index` is count of word entry in text
    ///     `value` is count of words, with same entry count
    ///
    pub fn graph(&self) -> Vec<u32> {
        let mut vec: Vec<u32> = Vec::new();

        for (_, v) in self.words_entry_map.iter() {
            let v = *v as usize;

            // If word entry count >= vec.len() than we need to
            // expand vec on (v - vec.len() + 1)
            if v >= vec.len() {
                vec.resize(v + 1, 0);
            }

            vec[v] += 1;
        }

        vec
    }
}


