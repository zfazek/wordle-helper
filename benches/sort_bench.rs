// Benchmark: previous O(n^2) sort vs. the shipping O(n) table-based sort.
//
// Run with: cargo bench
//
// `sort` is imported from the library crate, so this benchmark always measures
// the real production implementation (no risk of drift).
// `sort_slow` is a verbatim copy of the *previous* implementation, kept here
// only as a baseline for comparison.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordle_helper::filter::{rank_by_information, sort};

const WORDS: &str = include_str!("../words.txt");

fn load_words() -> Vec<String> {
    WORDS.lines().map(|l| l.to_owned()).collect()
}

/// Previous implementation: for each word, rescan the whole list 5 times.
fn sort_slow(words: &[String]) -> Vec<String> {
    let mut weights = words
        .iter()
        .map(|word| {
            (
                (0..5)
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

fn bench_sort(c: &mut Criterion) {
    let words = load_words();
    println!("benchmarking on {} words", words.len());

    let mut group = c.benchmark_group("sort");
    group.bench_function("slow_on2", |b| b.iter(|| sort_slow(black_box(&words))));
    // Real production implementation from the library crate.
    group.bench_function("fast_on", |b| b.iter(|| sort(black_box(&words))));
    group.finish();
}

/// The entropy/expected-remaining ranking is O(n^2) feedback simulations, so it
/// is far heavier than the positional sort. We measure it on a few candidate
/// set sizes to show how it scales as the list shrinks after each guess.
fn bench_rank_information(c: &mut Criterion) {
    let words = load_words();

    let mut group = c.benchmark_group("rank_by_information");
    // Reduce sample count: on the full list each call is expensive.
    group.sample_size(10);
    for &n in &[100usize, 500, 1000, words.len()] {
        let subset = &words[..n.min(words.len())];
        group.bench_function(format!("n_{}", subset.len()), |b| {
            b.iter(|| rank_by_information(black_box(subset)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_sort, bench_rank_information);
criterion_main!(benches);
