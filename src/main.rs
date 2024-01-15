use std::collections::BTreeMap;
use std::collections::BTreeSet;

struct Solution {
    words: Vec<String>,
    unknown_pos: BTreeMap<char, usize>,
    not_found_chars: BTreeSet<char>,
    known_pos: BTreeMap<usize, char>,
}

impl Solution {
    fn new() -> Self {
        let input = include_str!("../words.txt");
        let mut words = Vec::new();
        for line in input.lines() {
            words.push(line.to_owned());
        }
        let unknown_pos: BTreeMap<char, usize> = BTreeMap::new();
        let not_found_chars: BTreeSet<char> = BTreeSet::new();
        let known_pos: BTreeMap<usize, char> = BTreeMap::new();
        Solution {
            words,
            unknown_pos,
            not_found_chars,
            known_pos,
        }
    }

    fn list(&self) -> Vec<String> {
        let mut result = Vec::new();
        'iter: for word in self.words.iter() {
            for &c in self.not_found_chars.iter() {
                if word.contains(c) {
                    continue 'iter;
                }
            }
            for (&i, &c) in self.known_pos.iter() {
                if let Some(v) = word.find(c) {
                    if v != i - 1 {
                        continue 'iter;
                    }
                } else {
                    continue 'iter;
                }
            }
            for (&c, &i) in self.unknown_pos.iter() {
                if let Some(v) = word.find(c) {
                    if v == i - 1 {
                        continue 'iter;
                    }
                }
            }
            result.push(word.to_owned());
        }
        result
    }
}

fn main() {
    let mut my = Solution::new();
    println!("{}", my.words.len());
    //my.unknown_pos.insert('l', 4);
    //my.unknown_pos.insert('n', 4);
    //my.known_pos.insert(2, 'u');
    //my.known_pos.insert(3, 'n');
    my.known_pos.insert(4, 'c');
    my.known_pos.insert(5, 'h');
    my.not_found_chars
        .append(&mut BTreeSet::from(['p', 'f', 'y', 'a', 'i', 'd', 'o']));
    let words = my.list();
    println!("{:?}", words);
    println!("{}", words.len());
}
