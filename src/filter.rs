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
    let mut weights = words
        .iter()
        .map(|word| {
            (
                (0..4)
                    .map(|i| {
                        words
                            .iter()
                            .filter(move |&w| {
                                w.chars().nth(i).unwrap() == word.chars().nth(i).unwrap()
                            })
                            .count()
                    })
                    .sum::<usize>(),
                word,
            )
        })
        .collect::<Vec<_>>();
    weights.sort_by(|a, b| b.cmp(a));
    weights.iter().map(|x| x.1.to_owned()).collect()
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
