// build release:
// trunk build --release --dist release
//
// nvim release/index.html
//   :%s/wordle/wordle\/wordle/g
//
// build temp:
// trunk serve --release --address 0.0.0.0 --port 8000

use crate::filter::get_filtered_words;
use crate::filter::sort;
use leptos::prelude::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

mod filter;

const NUM_COLS: usize = 6;

#[component]
fn App() -> impl IntoView {
    let input = include_str!("../words.txt");
    let mut w = Vec::new();
    for line in input.lines() {
        w.push(line.to_owned());
    }
    w = sort(&w);
    let unknown_pos: BTreeMap<char, Vec<usize>> = BTreeMap::new();
    let not_found_chars: BTreeSet<char> = BTreeSet::new();
    let known_pos: BTreeMap<usize, char> = BTreeMap::new();
    let (words, _set_words) = signal(w.clone());
    let (filtered_words, set_filtered_words) = signal(w);
    let (not_found_chars, set_not_found_chars) = signal(not_found_chars);
    let (known_pos, set_known_pos) = signal(known_pos);
    let (unknown_pos, set_unknown_pos) = signal(unknown_pos);
    view! {
        <h1>Wordle Helper</h1>
        <table>
            <tr>
                <td>Letters which are not in the word (e.g., abdw) :</td>
                <td>
                    <input
                        type="text"
                        on:input=move |ev| {
                            let mut temp = not_found_chars.get();
                            temp.clear();
                            set_not_found_chars.set(temp);
                            let chars = event_target_value(&ev);
                            for c in chars.chars() {
                                if c.is_ascii_lowercase() || c.is_ascii_uppercase() {
                                    let c = c.to_ascii_lowercase();
                                    let mut temp = not_found_chars.get();
                                    temp.insert(c);
                                    set_not_found_chars.set(temp);
                                }
                            }
                            set_filtered_words
                                .set(
                                    get_filtered_words(
                                        &words.get(),
                                        &unknown_pos.get(),
                                        &not_found_chars.get(),
                                        &known_pos.get(),
                                    ),
                                );
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
                            let mut temp = unknown_pos.get();
                            temp.clear();
                            set_unknown_pos.set(temp);
                            let chars = event_target_value(&ev);
                            let chars = chars
                                .chars()
                                .filter(|x| {
                                    x.is_ascii_lowercase() || x.is_ascii_uppercase()
                                        || x.is_ascii_digit()
                                });
                            let mut it = chars;
                            while let Some(c) = it.next() {
                                if c.is_ascii_lowercase() || c.is_ascii_uppercase() {
                                    let c = c.to_ascii_lowercase();
                                    if let Some(i) = it.next() {
                                        if let Some(n) = i.to_digit(10) {
                                            let mut temp = unknown_pos.get();
                                            temp.entry(c).or_default().push(n as usize);
                                            set_unknown_pos.set(temp);
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                            set_filtered_words
                                .set(
                                    get_filtered_words(
                                        &words.get(),
                                        &unknown_pos.get(),
                                        &not_found_chars.get(),
                                        &known_pos.get(),
                                    ),
                                );
                        }
                    />

                </td>
            </tr>
            <tr>
                <td>Known letters:</td>
                <td>
                    <input
                        type="text"
                        size="1"
                        maxlength="1"
                        on:input=move |ev| {
                            let idx = 1;
                            let str = event_target_value(&ev);
                            filter_known_pos(&str, idx, known_pos, set_known_pos);
                            set_filtered_words
                                .set(
                                    get_filtered_words(
                                        &words.get(),
                                        &unknown_pos.get(),
                                        &not_found_chars.get(),
                                        &known_pos.get(),
                                    ),
                                );
                        }
                    />

                    <input
                        type="text"
                        size="1"
                        maxlength="1"
                        on:input=move |ev| {
                            let idx = 2;
                            let str = event_target_value(&ev);
                            filter_known_pos(&str, idx, known_pos, set_known_pos);
                            set_filtered_words
                                .set(
                                    get_filtered_words(
                                        &words.get(),
                                        &unknown_pos.get(),
                                        &not_found_chars.get(),
                                        &known_pos.get(),
                                    ),
                                );
                        }
                    />

                    <input
                        type="text"
                        size="1"
                        maxlength="1"
                        on:input=move |ev| {
                            let idx = 3;
                            let str = event_target_value(&ev);
                            filter_known_pos(&str, idx, known_pos, set_known_pos);
                            set_filtered_words
                                .set(
                                    get_filtered_words(
                                        &words.get(),
                                        &unknown_pos.get(),
                                        &not_found_chars.get(),
                                        &known_pos.get(),
                                    ),
                                );
                        }
                    />

                    <input
                        type="text"
                        size="1"
                        maxlength="1"
                        on:input=move |ev| {
                            let idx = 4;
                            let str = event_target_value(&ev);
                            filter_known_pos(&str, idx, known_pos, set_known_pos);
                            set_filtered_words
                                .set(
                                    get_filtered_words(
                                        &words.get(),
                                        &unknown_pos.get(),
                                        &not_found_chars.get(),
                                        &known_pos.get(),
                                    ),
                                );
                        }
                    />

                    <input
                        type="text"
                        size="1"
                        maxlength="1"
                        on:input=move |ev| {
                            let idx = 5;
                            let str = event_target_value(&ev);
                            filter_known_pos(&str, idx, known_pos, set_known_pos);
                            set_filtered_words
                                .set(
                                    get_filtered_words(
                                        &words.get(),
                                        &unknown_pos.get(),
                                        &not_found_chars.get(),
                                        &known_pos.get(),
                                    ),
                                );
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
            <td>
            <td>
        <ul>
            {move || {
                filtered_words.get().into_iter().enumerate().filter(|(i, _)| i % NUM_COLS == 0).map(|(_, n)| view! { <li>{n}</li> }).collect::<Vec<_>>()
            }}

        </ul>
            </td>
            <td>
        <ul>
            {move || {
                filtered_words.get().into_iter().enumerate().filter(|(i, _)| i % NUM_COLS == 1).map(|(_, n)| view! { <li>{n}</li> }).collect::<Vec<_>>()
            }}

        </ul>
            </td>
            <td>
        <ul>
            {move || {
                filtered_words.get().into_iter().enumerate().filter(|(i, _)| i % NUM_COLS == 2).map(|(_, n)| view! { <li>{n}</li> }).collect::<Vec<_>>()
            }}

        </ul>
            </td>
            </td>
            <td>
            <td>
        <ul>
            {move || {
                filtered_words.get().into_iter().enumerate().filter(|(i, _)| i % NUM_COLS == 3).map(|(_, n)| view! { <li>{n}</li> }).collect::<Vec<_>>()
            }}

        </ul>
            </td>
            <td>
        <ul>
            {move || {
                filtered_words.get().into_iter().enumerate().filter(|(i, _)| i % NUM_COLS == 4).map(|(_, n)| view! { <li>{n}</li> }).collect::<Vec<_>>()
            }}

        </ul>
            </td>
            <td>
        <ul>
            {move || {
                filtered_words.get().into_iter().enumerate().filter(|(i, _)| i % NUM_COLS == 5).map(|(_, n)| view! { <li>{n}</li> }).collect::<Vec<_>>()
            }}

        </ul>
            </td>
            </td>
            </tr>
        </table>
    }
}

fn filter_known_pos(
    str: &str,
    idx: usize,
    known_pos: ReadSignal<BTreeMap<usize, char>>,
    set_known_pos: WriteSignal<BTreeMap<usize, char>>,
) {
    if let Some(c) = str.chars().next() {
        if c.is_ascii_lowercase() || c.is_ascii_uppercase() {
            let c = c.to_ascii_lowercase();
            let mut temp = known_pos.get();
            temp.insert(idx, c);
            set_known_pos.set(temp);
        }
    } else {
        let mut temp = known_pos.get();
        temp.remove(&idx);
        set_known_pos.set(temp);
    }
}

fn main() {
    leptos::mount::mount_to_body(|| view! { <App/> });
}
