
extern crate libc;
extern crate walkdir;

pub use walkdir::WalkDir;

use std::collections::HashMap;

mod abi;
pub use abi::*;

mod help;
use help::*;

#[derive(Debug)]
pub struct WordSearcher {
    dictionary: HashMap<String, KeyWords>
}

impl WordSearcher {

    pub fn new(dir: &str) -> WordSearcher {
        let kws = WalkDir::new(dir).into_iter().skip(1)
            .map(|e| e.unwrap().path().to_str().unwrap().to_string())
            .map(|path| {

                let text = {
                    let ref_path = &*path;
                    read_text_from_file(ref_path)
                };

                (path, text)
            })
            .map(|(path, text)| (path, parse_text(text)))
            .map(|(path, kw)| (path, rearrange_keywords(kw)))
            .collect::<HashMap<String, KeyWords>>();

        WordSearcher {
            dictionary: kws
        }
    }

    pub fn search_string(&self, s: &str) -> Option<&str> {
        for (file, kw) in self.dictionary.iter() {
            if kw.contains(s) {
                return Some(&*file);
            }
        }
        None
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {

        println!("{:#?}", WordSearcher::new(".\\texts").search_string("text1"));

        assert!(false);
    }

}
