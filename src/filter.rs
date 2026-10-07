use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn get_filtered_words(
    words: &[String],
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
    sort(&result)
}

/// Ranks words by positional letter frequency: a word scores higher when its
/// letters are the ones most commonly seen at those same positions across the
/// whole list. Computed in O(n) via a precomputed `[position][letter]` table
/// instead of the previous O(n^2) per-word rescan.
pub fn sort(words: &[String]) -> Vec<String> {
    const WORD_LEN: usize = 5;
    const ALPHABET: usize = 26;

    // Phase A: one pass to tally how many words have letter `c` at position `i`.
    let mut counts = [[0u32; ALPHABET]; WORD_LEN];
    for word in words {
        for (i, b) in word.bytes().take(WORD_LEN).enumerate() {
            if b.is_ascii_lowercase() {
                counts[i][(b - b'a') as usize] += 1;
            }
        }
    }

    // Phase B: score each word with 5 table lookups, no rescan.
    let mut weights = words
        .iter()
        .map(|word| {
            let score: u32 = word
                .bytes()
                .take(WORD_LEN)
                .enumerate()
                .map(|(i, b)| {
                    if b.is_ascii_lowercase() {
                        counts[i][(b - b'a') as usize]
                    } else {
                        0
                    }
                })
                .sum();
            (score, word)
        })
        .collect::<Vec<_>>();

    // Highest score first.
    weights.sort_by(|a, b| b.cmp(a));
    weights.into_iter().map(|(_, w)| w.to_owned()).collect()
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
