// build release (served under /wordle/):
// trunk build --release --dist release --public-url /wordle/
//
// --public-url makes Trunk emit asset paths prefixed with /wordle/, so no
// manual edit of release/index.html is needed.
//
// serve locally (served at root /):
// trunk serve --release --address 0.0.0.0 --port 8000

use leptos::prelude::*;
use wordle_helper::filter::{filter_by_guesses, parse_pattern, sort, Guess, RankMethod};
use wordle_helper::filter::{GREEN, YELLOW};

const NUM_COLS: usize = 6;

#[component]
fn App() -> impl IntoView {
    // Both word lists are embedded at compile time. Switching between them is a
    // runtime toggle (no rebuild/redeploy, no filesystem — WASM has none).
    let parse = |input: &str| input.lines().map(|l| l.to_owned()).collect::<Vec<_>>();
    let compact_words = StoredValue::new(sort(&parse(include_str!("../words-compact.txt"))));
    let full_words = StoredValue::new(sort(&parse(include_str!("../words-full.txt"))));

    // The list of submitted guesses: each a (word, feedback-pattern) pair.
    let (guesses, set_guesses) = signal(Vec::<Guess>::new());
    // Pending input row (word + 5-char color code).
    let (pending_word, set_pending_word) = signal(String::new());
    let (pending_code, set_pending_code) = signal(String::new());

    // Word-set toggle: off = compact answer set (default), on = full guess set.
    let (use_full, set_use_full) = signal(false);
    // Ranking method toggle: off = fast positional sort (default), on = the
    // heavier expected-information ranking.
    let (use_entropy, set_use_entropy) = signal(false);

    // Derived, reactive candidate list.
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
        filter_by_guesses(&words.read_value(), &guesses.get(), method)
    });

    // Appends the pending row as a guess once both fields are complete. The
    // inputs are sanitized as the user types, so this only needs a length check.
    let submit_guess = move || {
        let word = pending_word.get();
        let code = pending_code.get();
        if word.len() != 5 {
            return;
        }
        if let Some(pattern) = parse_pattern(&code) {
            set_guesses.update(|g| g.push((word, pattern)));
            set_pending_word.set(String::new());
            set_pending_code.set(String::new());
        }
    };

    view! {
        <h1>Wordle Helper</h1>

        <p>
            "Enter each guess and the colors Wordle showed. Color code: "
            <b>g</b> " = green (right spot), " <b>y</b> " = yellow (wrong spot), "
            <b>x</b> " = grey (not in word). Example: guess " <code>crane</code>
            ", code " <code>xxyxg</code> "."
        </p>

        // Submitted guesses, each rendered as colored tiles with a remove button.
        <div>
            {move || {
                let gs = guesses.get();
                if gs.is_empty() {
                    view! { <p><i>"No guesses yet."</i></p> }.into_any()
                } else {
                    gs.into_iter()
                        .enumerate()
                        .map(|(i, (word, pattern))| {
                            let tiles = word
                                .chars()
                                .zip(pattern)
                                .map(|(ch, tile)| {
                                    let color = match tile {
                                        GREEN => "#6aaa64",
                                        YELLOW => "#c9b458",
                                        _ => "#787c7e", // GREY
                                    };
                                    let style = format!(
                                        "display:inline-block;width:1.6em;height:1.6em;\
                                         line-height:1.6em;text-align:center;margin:1px;\
                                         color:white;text-transform:uppercase;\
                                         font-weight:bold;background:{color};",
                                    );
                                    view! { <span style=style>{ch}</span> }
                                })
                                .collect::<Vec<_>>();
                            view! {
                                <div style="margin:2px 0;">
                                    {tiles}
                                    <button on:click=move |_| {
                                        set_guesses.update(|g| { g.remove(i); });
                                    }>"✕"</button>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                        .into_any()
                }
            }}
        </div>

        // Pending input row: 5-letter word + 5-char color code. Enter submits.
        // Both inputs are sanitized on every keystroke so only valid characters
        // can ever appear (letters for the guess; g/y/x for the code).
        <div style="margin:8px 0;">
            <input
                type="text"
                size="6"
                maxlength="5"
                placeholder="guess"
                prop:value=move || pending_word.get()
                on:input=move |ev| {
                    let clean: String = event_target_value(&ev)
                        .chars()
                        .filter(|c| c.is_ascii_alphabetic())
                        .map(|c| c.to_ascii_lowercase())
                        .take(5)
                        .collect();
                    set_pending_word.set(clean);
                }
                on:keydown=move |ev| {
                    if ev.key() == "Enter" {
                        submit_guess();
                    }
                }
            />
            <input
                type="text"
                size="6"
                maxlength="5"
                placeholder="gyxxg"
                prop:value=move || pending_code.get()
                on:input=move |ev| {
                    let clean: String = event_target_value(&ev)
                        .chars()
                        .filter_map(|c| match c.to_ascii_lowercase() {
                            'g' => Some('g'),
                            'y' => Some('y'),
                            'x' | 'b' | '.' | '-' => Some('x'),
                            _ => None,
                        })
                        .take(5)
                        .collect();
                    set_pending_code.set(clean);
                }
                on:keydown=move |ev| {
                    if ev.key() == "Enter" {
                        submit_guess();
                    }
                }
            />
            <button
                prop:disabled=move || {
                    pending_word.get().len() != 5 || pending_code.get().len() != 5
                }
                on:click=move |_| submit_guess()
            >
                "Add guess"
            </button>
            <button on:click=move |_| {
                set_guesses.set(Vec::new());
            }>"Clear all"</button>
        </div>

        <table>
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
                <td>"Number of words left: " {move || filtered_words.get().len()}</td>
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

fn main() {
    leptos::mount::mount_to_body(|| view! { <App/> });
}
