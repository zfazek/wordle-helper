// build release:
// build release (served under /wordle/):
// trunk build --release --dist release --public-url /wordle/
//
// --public-url makes Trunk emit asset paths prefixed with /wordle/, so no
// manual edit of release/index.html is needed.
//
// serve locally (served at root /):
// trunk serve --release --address 0.0.0.0 --port 8000

use leptos::prelude::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use wordle_helper::filter::get_filtered_words_with;
use wordle_helper::filter::sort;
use wordle_helper::filter::RankMethod;

const NUM_COLS: usize = 6;

#[component]
fn App() -> impl IntoView {
    // Both word lists are embedded at compile time. Switching between them is a
    // runtime toggle (no rebuild/redeploy, no filesystem — WASM has none).
    let parse = |input: &str| input.lines().map(|l| l.to_owned()).collect::<Vec<_>>();
    let compact_words = StoredValue::new(sort(&parse(include_str!("../words-compact.txt"))));
    let full_words = StoredValue::new(sort(&parse(include_str!("../words-full.txt"))));

    let (not_found_chars, set_not_found_chars) = signal(BTreeSet::<char>::new());
    let (known_pos, set_known_pos) = signal(BTreeMap::<usize, char>::new());
    let (unknown_pos, set_unknown_pos) = signal(BTreeMap::<char, Vec<usize>>::new());
    // Word-set toggle: off = compact answer set (default), on = full guess set.
    let (use_full, set_use_full) = signal(false);
    // Ranking method toggle: off = fast positional sort (default), on = the
    // heavier expected-information ranking.
    let (use_entropy, set_use_entropy) = signal(false);
    // Derived, reactive list: recomputes whenever any input signal changes.
    let filtered_words = Memo::new(move |_| {
        let full = use_full.get();
        // The entropy ranking is O(n^2); it is unusable on the full (~13k) set,
        // so force the fast positional sort whenever the full set is active.
        let method = if use_entropy.get() && !full {
            RankMethod::Information
        } else {
            RankMethod::Positional
        };
        let words = if full { full_words } else { compact_words };
        get_filtered_words_with(
            &words.read_value(),
            &unknown_pos.get(),
            &not_found_chars.get(),
            &known_pos.get(),
            method,
        )
    });
    view! {
        <h1>Wordle Helper</h1>
        <table>
            <tr>
                <td>Letters which are not in the word (e.g., abdw) :</td>
                <td>
                    <input
                        type="text"
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_not_found_chars
                                .update(|chars| {
                                    chars.clear();
                                    chars
                                        .extend(
                                            value
                                                .chars()
                                                .filter(|c| c.is_ascii_alphabetic())
                                                .map(|c| c.to_ascii_lowercase()),
                                        );
                                });
                        }
                    />

                </td>
            </tr>
            <tr>
                <td>Letters which are not in the right position (e.g.,a1b2a3d5) :</td>
                <td>
                    <input
                        type="text"
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_unknown_pos
                                .update(|map| {
                                    map.clear();
                                    let mut it = value
                                        .chars()
                                        .filter(|x| x.is_ascii_alphabetic() || x.is_ascii_digit());
                                    while let Some(c) = it.next() {
                                        if c.is_ascii_alphabetic() {
                                            let c = c.to_ascii_lowercase();
                                            if let Some(i) = it.next() {
                                                if let Some(n) = i.to_digit(10) {
                                                    map.entry(c).or_default().push(n as usize);
                                                }
                                            } else {
                                                break;
                                            }
                                        }
                                    }
                                });
                        }
                    />

                </td>
            </tr>
            <tr>
                <td>Known letters:</td>
                <td>
                    {(1..=5)
                        .map(|idx| {
                            view! {
                                <input
                                    type="text"
                                    size="1"
                                    maxlength="1"
                                    on:input=move |ev| {
                                        let str = event_target_value(&ev);
                                        filter_known_pos(&str, idx, set_known_pos);
                                    }
                                />
                            }
                        })
                        .collect::<Vec<_>>()}
                </td>
            </tr>
            <tr>
                <td>"Use full word set (bigger; use when the answer is missing):"</td>
                <td>
                    <input
                        type="checkbox"
                        prop:checked=move || use_full.get()
                        on:change=move |ev| {
                            set_use_full.set(event_target_checked(&ev));
                        }
                    />
                </td>
            </tr>
            <tr>
                <td>"Rank by expected information (slower, better guesses):"</td>
                <td>
                    <input
                        type="checkbox"
                        prop:checked=move || use_entropy.get()
                        prop:disabled=move || use_full.get()
                        on:change=move |ev| {
                            set_use_entropy.set(event_target_checked(&ev));
                        }
                    />
                </td>
            </tr>
            <tr>
            <td>
        Number of words left: {move || filtered_words.get().len()}
            </td>
            </tr>
            </table>
            <table>
            <tr>
                {(0..NUM_COLS)
                    .map(|col| {
                        view! {
                            <td>
                                <ul>
                                    {move || {
                                        filtered_words
                                            .get()
                                            .into_iter()
                                            .skip(col)
                                            .step_by(NUM_COLS)
                                            .map(|n| view! { <li>{n}</li> })
                                            .collect::<Vec<_>>()
                                    }}
                                </ul>
                            </td>
                        }
                    })
                    .collect::<Vec<_>>()}
            </tr>
        </table>
    }
}

fn filter_known_pos(
    str: &str,
    idx: usize,
    set_known_pos: WriteSignal<BTreeMap<usize, char>>,
) {
    match str.chars().next() {
        Some(c) if c.is_ascii_alphabetic() => {
            let c = c.to_ascii_lowercase();
            set_known_pos.update(|map| {
                map.insert(idx, c);
            });
        }
        None => {
            set_known_pos.update(|map| {
                map.remove(&idx);
            });
        }
        _ => {}
    }
}

fn main() {
    leptos::mount::mount_to_body(|| view! { <App/> });
}
