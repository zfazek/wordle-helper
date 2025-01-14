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
            let n = get_num_chars_in_pos_filters(c, unknown_pos, known_pos);
            let m = word.matches(c).count();
            if m > n {
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
        for (&c, indices) in unknown_pos.iter() {
            if !word.contains(c) {
                continue 'iter;
            }
            for &i in indices {
                let v = word.chars().nth(i - 1).unwrap();
                if v == c {
                    continue 'iter;
                }
            }
        }
        result.push(word.to_owned());
    }
    let result = sort(&result);
    result
}

pub(crate) fn sort(words: &Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    let mut weights = Vec::new();
    for word in words.iter() {
        let mut weight: usize = 0;
        for i in 0..4 {
            let c = word.chars().nth(i).unwrap();
            for w in words.iter() {
                if w.chars().nth(i).unwrap() == c {
                    weight += 1;
                }
            }
        }
        weights.push((weight, word));
    }
    weights.sort_by(|a, b| b.cmp(a));
    for w in weights {
        res.push(w.1.to_owned());
    }
    res
}

fn get_num_chars_in_pos_filters(
    c: char,
    unknown_pos: &BTreeMap<char, Vec<usize>>,
    known_pos: &BTreeMap<usize, char>,
) -> usize {
    let mut count = 0;
    if let Some(v) = unknown_pos.get(&c) {
        count += v.len();
    }
    count += known_pos.values().filter(|&x| c == *x).count();
    count
}
