use leptos::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn list(
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
            if let Some(v) = word.find(c) {
                if v != i - 1 {
                    continue 'iter;
                }
            } else {
                continue 'iter;
            }
        }
        for (&c, ids) in unknown_pos.iter() {
            for &i in ids {
                if let Some(v) = word.find(c) {
                    if v == i - 1 {
                        continue 'iter;
                    }
                } else {
                    continue 'iter;
                }
            }
        }
        result.push(word.to_owned());
    }
    result
}

#[component]
fn App() -> impl IntoView {
    let input = include_str!("../words.txt");
    let mut w = Vec::new();
    for line in input.lines() {
        w.push(line.to_owned());
    }
    let unknown_pos: BTreeMap<char, Vec<usize>> = BTreeMap::new();
    let not_found_chars: BTreeSet<char> = BTreeSet::new();
    let known_pos: BTreeMap<usize, char> = BTreeMap::new();
    let (words, _set_words) = create_signal(w.clone());
    let (filtered_words, set_filtered_words) = create_signal(w);
    let (not_found_chars, set_not_found_chars) = create_signal(not_found_chars);
    let (known_pos, set_known_pos) = create_signal(known_pos);
    let (unknown_pos, set_unknown_pos) = create_signal(unknown_pos);
    view! {
        <p>Letters which are not in the word:
        <input type="text"
            on:input=move |ev| {
                let mut temp = not_found_chars.get();
                temp.clear();
                set_not_found_chars.set(temp);
                let chars = event_target_value(&ev);
                for c in chars.chars() {
                    if c.is_ascii_lowercase() {
                      let mut temp = not_found_chars.get();
                      temp.insert(c);
                      set_not_found_chars.set(temp);
                    }
                }
                set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
            }
        /></p>
        <p>Letters which are not in the right position:
        <input type="text"
            on:input=move |ev| {
                let mut temp = unknown_pos.get();
                temp.clear();
                set_unknown_pos.set(temp);
                let chars = event_target_value(&ev);
                let mut it = chars.chars();
                while let Some(c) = it.next() {
                    if c.is_ascii_lowercase() {
                        if let Some(i) = it.next() {
                            if let Some(n) = i.to_digit(10) {
                                let mut temp = unknown_pos.get();
                                //temp.insert(c, n as usize);
                                temp.entry(c).or_default().push(n as usize);
                                set_unknown_pos.set(temp);
                            }
                        } else {
                            break;
                        }
                    }
                }
                set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
            }
        /></p>
        <p>Known letters:
        <input type="text" size="1" maxlength="1"
            on:input=move |ev| {
                let idx = 1;
                let str = event_target_value(&ev);
                if !str.is_empty() {
                    let c1 = str.chars().next().unwrap();
                    let mut temp = known_pos.get();
                    temp.insert(idx, c1);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                } else if str.is_empty() {
                    let mut temp = known_pos.get();
                    temp.remove(&idx);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                }
            }
        />
        <input type="text" size="1" maxlength="1"
            on:input=move |ev| {
                let idx = 2;
                let str = event_target_value(&ev);
                if !str.is_empty() {
                    let c1 = str.chars().next().unwrap();
                    let mut temp = known_pos.get();
                    temp.insert(idx, c1);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                } else if str.is_empty() {
                    let mut temp = known_pos.get();
                    temp.remove(&idx);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                }
            }
        />
        <input type="text" size="1" maxlength="1"
            on:input=move |ev| {
                let idx = 3;
                let str = event_target_value(&ev);
                if !str.is_empty() {
                    let c1 = str.chars().next().unwrap();
                    let mut temp = known_pos.get();
                    temp.insert(idx, c1);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                } else if str.is_empty() {
                    let mut temp = known_pos.get();
                    temp.remove(&idx);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                }
            }
        />
        <input type="text" size="1" maxlength="1"
            on:input=move |ev| {
                let idx = 4;
                let str = event_target_value(&ev);
                if !str.is_empty() {
                    let c1 = str.chars().next().unwrap();
                    let mut temp = known_pos.get();
                    temp.insert(idx, c1);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                } else if str.is_empty() {
                    let mut temp = known_pos.get();
                    temp.remove(&idx);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                }
            }
        />
        <input type="text" size="1" maxlength="1"
            on:input=move |ev| {
                let idx = 5;
                let str = event_target_value(&ev);
                if !str.is_empty() {
                    let c1 = str.chars().next().unwrap();
                    let mut temp = known_pos.get();
                    temp.insert(idx, c1);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                } else if str.is_empty() {
                    let mut temp = known_pos.get();
                    temp.remove(&idx);
                    set_known_pos.set(temp);
                    set_filtered_words.set(list(&words.get(), &unknown_pos.get(), &not_found_chars.get(), &known_pos.get()));
                }
            }
        />
        </p>
        <p>Number of words left: {move || filtered_words.get().len()}</p>
        <ul>
            {move || filtered_words.get().into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect::<Vec<_>>()}
        </ul>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> });
}
