
use std::collections::HashMap;

///
/// Count words entry in text
///
/// Return:
///     count of words in text
///     HashMap witch contains word and count of this words in text
///
pub fn parse_text(text: String) -> (usize, HashMap<String, u32>) {
    let mut hash: HashMap<String, u32> = HashMap::new();

    let no_chars: &[char] = &['.', ','];
    let words = text.split_whitespace()
        .map(|s| s.trim_matches(no_chars).to_owned())
        .collect::<Vec<_>>();

    let text_length = words.len();

    for word in words {
        if hash.contains_key(&word) {
            if let Some(counter) = hash.get_mut(&word) {
                *counter += 1;
            }
        } else {
            hash.insert(word, 0);
        }
    }

    (text_length, hash)
}

///
/// `dedup` function like in `std` library but apply function on elements
/// to compare them. This provides a control over item comparison
/// and if item has not trivial type like tuple, this can be more flexible.
///
/// Function `f` must return true if items is equal.
///
pub fn dedup_by<T, F>(v: &mut Vec<T>, f: F) where F: Fn(&T, &T) -> bool {

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
