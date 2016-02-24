
use std::collections::HashMap;
use std::io::{ Read, BufReader };
use std::fs::File;

#[derive(Debug)]
pub struct KeyWords(Vec<(String, u32)>);

impl KeyWords {

    pub fn contains(&self, s: &str) -> bool {
        for e in self.0.iter() {
            if e.0 == s {
                return true;
            }
        }
        false
    }

}

pub fn read_text_from_file(path: &str) -> String {
    let mut reader = BufReader::new(File::open(path).expect("Failed to open file"));
    let mut text = String::new();

    reader.read_to_string(&mut text)
          .expect("Failed to read file");

    text
}

///
/// Count words entry in text
///
/// Return:
///     count of words in text
///     HashMap witch contains word and count of this words in text
///
pub fn parse_text(text: String) -> HashMap<String, u32> {
    let mut hash: HashMap<String, u32> = HashMap::new();

    let no_chars: &[char] = &['.', ','];
    let words = text.split_whitespace()
        .map(|s| s.trim_matches(no_chars).to_owned())
        .collect::<Vec<_>>();

    for word in words {
        if hash.contains_key(&word) {
            if let Some(counter) = hash.get_mut(&word) {
                *counter += 1;
            }
        } else {
            hash.insert(word, 0);
        }
    }

    hash
}

///
/// Converts HashMap to Vec<_> to sort by rang of words
/// In Vec<_> index represents rang of word in [Zippha theorem]
///
pub fn rearrange_keywords(mut keywords: HashMap<String, u32>) -> KeyWords {
    let mut v = keywords.drain().collect::<Vec<_>>();

    v.sort_by(|&(_, v1), &(_, v2)| v1.cmp(&v2).reverse());

    KeyWords(v)
}


