use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub(crate) fn get_filtered_words(
    words: &Vec<String>,
    unknown_pos: &BTreeMap<char, Vec<usize>>,
    not_found_chars: &BTreeSet<char>,
    known_pos: &BTreeMap<usize, char>,
) -> Vec<String> {
    let mut result = Vec::new();
    'iter: for word in words.iter() {
        for &c in not_found_chars.iter() {
            if word.contains(c) {
                continue 'iter;
            }
        }
        for (&i, &c) in known_pos.iter() {
            if !word.contains(c) {
                continue 'iter;
            }
            let v = word.chars().nth(i - 1).unwrap();
            if v != c {
                continue 'iter;
            }
        }
        for (&c, ids) in unknown_pos.iter() {
            if !word.contains(c) {
                continue 'iter;
            }
            for &i in ids {
                let v = word.chars().nth(i - 1).unwrap();
                if v == c {
                    continue 'iter;
                }
            }
        }
        result.push(word.to_owned());
    }
    result
}
