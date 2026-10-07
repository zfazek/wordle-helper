// build release:
// trunk build --release --dist release
//
// nvim release/index.html
//   :%s/wordle/wordle\/wordle/g
//
// build temp:
// trunk serve --release --address 0.0.0.0 --port 8000

use leptos::prelude::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use wordle_helper::filter::get_filtered_words;
use wordle_helper::filter::sort;

const NUM_COLS: usize = 6;

#[component]
fn App() -> impl IntoView {
    let input = include_str!("../words.txt");
    let mut w = Vec::new();
    for line in input.lines() {
        w.push(line.to_owned());
    }
    w = sort(&w);
    let words = StoredValue::new(w);
    let (not_found_chars, set_not_found_chars) = signal(BTreeSet::<char>::new());
    let (known_pos, set_known_pos) = signal(BTreeMap::<usize, char>::new());
    let (unknown_pos, set_unknown_pos) = signal(BTreeMap::<char, Vec<usize>>::new());
    // Derived, reactive list: recomputes whenever any input signal changes.
    let filtered_words = Memo::new(move |_| {
        get_filtered_words(
            &words.read_value(),
            &unknown_pos.get(),
            &not_found_chars.get(),
            &known_pos.get(),
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
