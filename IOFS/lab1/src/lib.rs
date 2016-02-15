
extern crate libc;

use std::collections::HashMap;
use std::io::{ Read, BufReader };
use std::fs::File;

mod abi;
pub use abi::*;

pub struct WordCounter(String);

impl Drop for WordCounter {
    fn drop(&mut self) {
        println!("Object has been dropped!");
    }
}

impl WordCounter {

    pub fn new(path: &str) -> WordCounter {
        let mut reader = BufReader::new(File::open(path).expect("Failed to open file"));
        let mut text = String::new();

        reader.read_to_string(&mut text)
              .expect("Failed to read file");

        WordCounter(text)
    }

    ///
    /// Count words entry in text
    ///
    /// Return:
    ///     count of words in text
    ///     HashMap witch contains word and count of this word in text
    ///
    fn get_entries(&self) -> (usize, HashMap<&str, u32>) {
        let mut hash: HashMap<&str, u32> = HashMap::new();

        let no_chars: &[char] = &['.', ','];
        let words = self.0.split_whitespace()
            .map(|s| s.trim_matches(no_chars))
            .collect::<Vec<_>>();

        let text_length = words.len();

        for word in words {
            if hash.contains_key(word) {
                if let Some(counter) = hash.get_mut(word) {
                    *counter += 1;
                }
            } else {
                hash.insert(word, 0);
            }
        }

        (text_length, hash)
    }

    ///
    /// Calculates probability of each word in text
    ///
    /// Converts HashMap to Vec<_> to sort by probability of words
    /// In Vec<_> index represents rang of word in [Zippha theorem]
    ///
    pub fn count_probability(&self) -> Vec<(&str, f32)> {
        let (text_length, mut entries) = self.get_entries();

        // Pi = Ni / N
        let mut v = entries.drain()
            .map(|(k, v)| (k, v as f32 / text_length as f32))
            .collect::<Vec<(&str, f32)>>();

        // Sort to make words with max Pi have max rang
        v.sort_by(|&(_, v1), &(_, v2)|
            v1.partial_cmp(&v2).unwrap().reverse() // FIXME: Use unwrap() - may fail
        );

        dedup_by(&mut v, |&(_, v1), &(_, v2)| v1 == v2);

        v
    }
}

///
/// `dedup` function like in `std` library but apply function on elements
/// to compare them. This provides a control over item comparison
/// and if item has not trivial type like tuple, this can be more flexible.
///
/// Function `f` must return true if items is equal.
///
fn dedup_by<T, F>(v: &mut Vec<T>, f: F) where F: Fn(&T, &T) -> bool {

    let len = v.len();

    if len <= 1 { return; }

    let pv = v.as_mut_ptr();

    let mut r: usize = 1;
    let mut w: usize = 1;

    unsafe {
        while r < len {
            let pr = pv.offset(r as isize);
            let mut pw = pv.offset((w - 1) as isize);

            if !f(&*pr, &*pw) {
                if r != w {
                    pw = pw.offset(1);
                    ::std::mem::swap(&mut *pw, &mut *pr);
                }
                w += 1;
            }
            r += 1;
        }

        v.truncate(w);
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sort() {
        let wc = WordCounter::new();
        wc.count_probability();
        assert!(false);
    }

    #[test]
    fn test_abi() {

        let ptr = CreateWordCounter();
        DeleteWordCounter(ptr);

        assert!(false);
    }

    #[test]
    fn test_enumerate() {

        let ptr = CreateWordCounter();

        fn print(r: u32, y: f32) { println!("Rang: {} Value: {}", r, y); }

        Enumerate(ptr, print);

        DeleteWordCounter(ptr);
        assert!(false);
    }

}
