/// Which ranking heuristic to apply to the filtered candidate list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RankMethod {
    /// Fast O(n) positional letter-frequency score (see [`sort`]).
    Positional,
    /// O(n^2) expected-information ranking (see [`rank_by_information`]).
    Information,
}

/// Ranks words by a blend of positional and presence letter frequency.
///
/// Each word's score combines two O(n) precomputed tables:
/// * positional ("green" value): for each of the 5 slots, how many words share
///   that letter in that slot;
/// * presence ("yellow" value): for each *distinct* letter in the word, how many
///   words contain that letter anywhere.
///
/// Counting presence per distinct letter (not per occurrence) rewards broad
/// letter coverage without over-crediting duplicate letters. Still O(n) overall.
pub fn sort(words: &[String]) -> Vec<String> {
    const WORD_LEN: usize = 5;
    const ALPHABET: usize = 26;

    // Phase A: tally positional counts and per-letter presence (once per word).
    let mut counts = [[0u32; ALPHABET]; WORD_LEN];
    let mut present = [0u32; ALPHABET];
    for word in words {
        let mut seen = [false; ALPHABET];
        for (i, b) in word.bytes().take(WORD_LEN).enumerate() {
            if b.is_ascii_lowercase() {
                let l = (b - b'a') as usize;
                counts[i][l] += 1;
                if !seen[l] {
                    seen[l] = true;
                    present[l] += 1;
                }
            }
        }
    }

    // Phase B: score each word = positional (green) + distinct-presence (yellow).
    let mut weights = words
        .iter()
        .map(|word| {
            let mut seen = [false; ALPHABET];
            let mut score: u32 = 0;
            for (i, b) in word.bytes().take(WORD_LEN).enumerate() {
                if b.is_ascii_lowercase() {
                    let l = (b - b'a') as usize;
                    score += counts[i][l];
                    if !seen[l] {
                        seen[l] = true;
                        score += present[l];
                    }
                }
            }
            (score, word)
        })
        .collect::<Vec<_>>();

    // Highest score first.
    weights.sort_by(|a, b| b.cmp(a));
    weights.into_iter().map(|(_, w)| w.to_owned()).collect()
}

/// Tile colors returned by [`feedback`]: grey (not in word), yellow (in word,
/// wrong position), green (correct position).
pub const GREY: u8 = 0;
pub const YELLOW: u8 = 1;
pub const GREEN: u8 = 2;

/// Simulates the Wordle feedback a `guess` would produce against a hypothetical
/// `answer`, using the correct two-pass algorithm for duplicate letters.
///
/// Returns a 5-element array of tile states ([`GREEN`]/[`YELLOW`]/[`GREY`]).
/// Both inputs are assumed to be 5 ASCII-lowercase letters.
pub fn feedback(guess: &str, answer: &str) -> [u8; 5] {
    let g = guess.as_bytes();
    let a = answer.as_bytes();
    let mut result = [GREY; 5];

    // Count of each answer letter still available to match (not yet consumed).
    let mut remaining = [0u8; 26];

    // Pass 1: greens. Consume the matched answer letters; the rest stay in the
    // pool for yellow matching.
    for i in 0..5 {
        if g[i] == a[i] {
            result[i] = GREEN;
        } else if a[i].is_ascii_lowercase() {
            remaining[(a[i] - b'a') as usize] += 1;
        }
    }

    // Pass 2: yellows. A non-green tile is yellow only while an unconsumed
    // occurrence of that letter remains in the answer.
    for i in 0..5 {
        if result[i] == GREEN || !g[i].is_ascii_lowercase() {
            continue;
        }
        let idx = (g[i] - b'a') as usize;
        if remaining[idx] > 0 {
            result[i] = YELLOW;
            remaining[idx] -= 1;
        }
    }

    result
}

/// A past guess together with the feedback pattern the player observed for it.
pub type Guess = (String, [u8; 5]);

/// Parses a 5-character color code into a feedback pattern.
///
/// Accepted characters (case-insensitive):
/// * `g` -> [`GREEN`]
/// * `y` -> [`YELLOW`]
/// * `x`, space, `.`, or `-` -> [`GREY`]
///
/// Returns `None` unless the code is exactly 5 valid characters.
pub fn parse_pattern(code: &str) -> Option<[u8; 5]> {
    let mut pattern = [GREY; 5];
    let mut n = 0;
    for (i, c) in code.chars().enumerate() {
        if i >= 5 {
            return None; // too long
        }
        pattern[i] = match c.to_ascii_lowercase() {
            'g' => GREEN,
            'y' => YELLOW,
            'x' | ' ' | '.' | '-' => GREY,
            _ => return None,
        };
        n += 1;
    }
    if n == 5 {
        Some(pattern)
    } else {
        None
    }
}

/// True iff `candidate` is consistent with the observed `pattern` for `guess`,
/// i.e. guessing `guess` against `candidate` would reproduce exactly `pattern`.
pub fn matches_feedback(candidate: &str, guess: &str, pattern: &[u8; 5]) -> bool {
    feedback(guess, candidate) == *pattern
}

/// Filters `words` to those consistent with every observed guess, then ranks
/// the survivors with `method`.
///
/// A word survives iff, for every `(guess, pattern)`, simulating the guess
/// against the word reproduces the observed pattern. This is exactly Wordle's
/// own feedback rule, so it handles all duplicate-letter cases correctly
/// (unlike attribute buckets, it preserves per-guess letter counts).
pub fn filter_by_guesses(words: &[String], guesses: &[Guess], method: RankMethod) -> Vec<String> {
    let result: Vec<String> = words
        .iter()
        .filter(|word| {
            guesses
                .iter()
                .all(|(guess, pattern)| matches_feedback(word, guess, pattern))
        })
        .cloned()
        .collect();
    match method {
        RankMethod::Positional => sort(&result),
        RankMethod::Information => rank_by_information(&result),
    }
}

/// Encodes a feedback pattern as a base-3 integer in `0..243`, for use as a
/// bucket index.
fn pattern_index(pattern: &[u8; 5]) -> usize {
    pattern
        .iter()
        .fold(0usize, |acc, &tile| acc * 3 + tile as usize)
}

/// Ranks words by expected information gain (the "entropy" method).
///
/// For each candidate guess, the remaining `candidates` are bucketed by the
/// feedback pattern the guess would produce. A guess that splits the candidates
/// into many small buckets narrows the search faster. We score each guess by
/// the expected number of remaining candidates, `sum(bucket_size^2) / N`
/// (smaller is better), and return the words best-first.
///
/// Cost is O(n^2) feedback simulations, which is acceptable on the shrinking
/// candidate set after each guess.
pub fn rank_by_information(candidates: &[String]) -> Vec<String> {
    let n = candidates.len();
    if n <= 1 {
        return candidates.to_vec();
    }

    let mut scored: Vec<(u64, &String)> = candidates
        .iter()
        .map(|guess| {
            let mut buckets = [0u32; 243];
            for answer in candidates {
                let idx = pattern_index(&feedback(guess, answer));
                buckets[idx] += 1;
            }
            // Expected remaining is sum(s^2)/n; n is constant across guesses, so
            // ranking by sum(s^2) alone gives the same order. Smaller is better.
            let sum_sq: u64 = buckets.iter().map(|&s| (s as u64) * (s as u64)).sum();
            (sum_sq, guess)
        })
        .collect();

    // Smallest expected-remaining first. Tie-break alphabetically for stable,
    // deterministic output.
    scored.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(b.1)));
    scored.into_iter().map(|(_, w)| w.to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_rewards_distinct_coverage_over_duplicates() {
        // "abcde" uses 5 distinct letters; "abcda" wastes its last slot on a
        // repeated 'a'. With equal-ish positional support, the distinct-letter
        // word should rank first under the presence blend.
        let words: Vec<String> = ["abcde", "abcda", "fghij"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ranked = sort(&words);
        let p_distinct = ranked.iter().position(|w| w == "abcde").unwrap();
        let p_dup = ranked.iter().position(|w| w == "abcda").unwrap();
        assert!(
            p_distinct < p_dup,
            "distinct-letter word should outrank the duplicate one: {ranked:?}"
        );
    }

    #[test]
    fn sort_is_stable_and_returns_all_words() {
        let words: Vec<String> = ["crane", "slate", "trace"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ranked = sort(&words);
        assert_eq!(ranked.len(), words.len());
        // Every input word is present in the output.
        for w in &words {
            assert!(ranked.contains(w));
        }
    }

    #[test]
    fn feedback_all_green_on_exact_match() {
        assert_eq!(feedback("crane", "crane"), [GREEN; 5]);
    }

    #[test]
    fn feedback_all_grey_when_no_overlap() {
        assert_eq!(feedback("fghij", "crane"), [GREY, GREY, GREY, GREY, GREY]);
    }

    #[test]
    fn feedback_simple_yellows_and_greens() {
        // answer SLATE, guess STALE:
        // S green; T present (pos4) -> yellow; A green; L present (pos1) -> yellow; E green.
        assert_eq!(
            feedback("stale", "slate"),
            [GREEN, YELLOW, GREEN, YELLOW, GREEN]
        );
    }

    #[test]
    fn feedback_duplicate_guess_letter_only_one_in_answer() {
        // answer ALLOY, guess LLAMA (from the explanation):
        // L yellow, L green, A yellow, M grey, A grey (second A has no match left).
        assert_eq!(
            feedback("llama", "alloy"),
            [YELLOW, GREEN, YELLOW, GREY, GREY]
        );
    }

    #[test]
    fn feedback_duplicate_green_consumes_before_yellow() {
        // answer EERIE, guess EAGER:
        // pos0 E == E -> green (consume one E)
        // pos1 A -> grey
        // pos2 G -> grey
        // pos3 E: answer has another E (EERIE has 3 E's, one consumed) -> yellow
        // pos4 R: EERIE has an R -> yellow
        assert_eq!(
            feedback("eager", "eerie"),
            [GREEN, GREY, GREY, YELLOW, YELLOW]
        );
    }

    #[test]
    fn feedback_extra_duplicate_goes_grey() {
        // answer ABIDE (one B), guess BOBBY:
        // pos0 B -> yellow (consume the single B)
        // pos1 O -> grey
        // pos2 B -> grey (no B left)
        // pos3 B -> grey
        // pos4 Y -> grey
        assert_eq!(
            feedback("bobby", "abide"),
            [YELLOW, GREY, GREY, GREY, GREY]
        );
    }

    #[test]
    fn pattern_index_is_unique_per_pattern() {
        assert_eq!(pattern_index(&[GREY; 5]), 0);
        assert_eq!(pattern_index(&[GREEN; 5]), 242); // 2*(81+27+9+3+1)=242
        assert_ne!(
            pattern_index(&[GREEN, GREY, GREY, GREY, GREY]),
            pattern_index(&[GREY, GREY, GREY, GREY, GREEN])
        );
    }

    #[test]
    fn rank_by_information_handles_small_inputs() {
        assert!(rank_by_information(&[]).is_empty());
        let one = vec!["crane".to_string()];
        assert_eq!(rank_by_information(&one), one);
    }

    #[test]
    fn rank_by_information_prefers_better_splitter() {
        // "aaaaa" produces the same feedback against every candidate here, so it
        // splits nothing; a word with distinct common letters splits better and
        // should rank ahead of it.
        let words: Vec<String> = ["slate", "crane", "trace", "aaaaa"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ranked = rank_by_information(&words);
        let pos_aaaaa = ranked.iter().position(|w| w == "aaaaa").unwrap();
        // The non-degenerate words should all out-rank the all-same-letter word.
        assert_eq!(pos_aaaaa, ranked.len() - 1);
    }

    #[test]
    fn feedback_is_consistent_with_itself() {
        // A word guessed against itself is always all green.
        for w in ["abcde", "hello", "zzzzz"] {
            assert_eq!(feedback(w, w), [GREEN; 5]);
        }
    }

    #[test]
    fn parse_pattern_valid_and_invalid() {
        assert_eq!(parse_pattern("ggggg"), Some([GREEN; 5]));
        assert_eq!(parse_pattern("xxxxx"), Some([GREY; 5]));
        assert_eq!(
            parse_pattern("gyxGY"),
            Some([GREEN, YELLOW, GREY, GREEN, YELLOW])
        );
        // Alternate grey spellings (space, dot, dash).
        assert_eq!(parse_pattern(" .-xy"), Some([GREY, GREY, GREY, GREY, YELLOW]));
        // Wrong length / invalid chars.
        assert_eq!(parse_pattern("gggg"), None);
        assert_eq!(parse_pattern("gggggg"), None);
        assert_eq!(parse_pattern("gg?gg"), None);
        assert_eq!(parse_pattern(""), None);
    }

    fn words(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn filter_by_guesses_empty_returns_all_ranked() {
        let all = words(&["crane", "slate", "trace"]);
        let out = filter_by_guesses(&all, &[], RankMethod::Positional);
        assert_eq!(out.len(), all.len());
    }

    #[test]
    fn filter_by_guesses_basic_greens_and_greys() {
        let all = words(&["slate", "crane", "trace", "plate", "blame"]);
        // Guess CRANE, answer-consistent pattern for SLATE:
        // C grey, R grey, A yellow(not pos3), N grey, E green.
        let pat = feedback("crane", "slate");
        let out = filter_by_guesses(&all, &[("crane".to_string(), pat)], RankMethod::Positional);
        // SLATE must survive (it generated the pattern); CRANE must not.
        assert!(out.contains(&"slate".to_string()));
        assert!(!out.contains(&"crane".to_string()));
        // Every survivor must reproduce the pattern.
        for w in &out {
            assert_eq!(feedback("crane", w), pat);
        }
    }

    #[test]
    fn filter_by_guesses_distinguishes_duplicate_letter_counts() {
        // The ambiguity the attribute-bucket model could not express:
        // "exactly one E and not at this position" vs "at least two E's".
        let all = words(&["abbey", "ebony", "elbow", "steel", "sheen"]);

        // Guess "eexxx"-style: use guess "eerie" against answer "ebony"
        // (one E, at position 0). Pattern: E green, E grey, R grey, I grey, E grey.
        let guess = "eerie";
        let pat_one_e = feedback(guess, "ebony");
        assert_eq!(pat_one_e, [GREEN, GREY, GREY, GREY, GREY]);
        let out_one = filter_by_guesses(
            &all,
            &[(guess.to_string(), pat_one_e)],
            RankMethod::Positional,
        );
        // "ebony" (one E) survives; "steel"/"sheen" (two E's) must be rejected,
        // because a second E would have shown yellow, not grey.
        assert!(out_one.contains(&"ebony".to_string()));
        assert!(!out_one.contains(&"steel".to_string()));
        assert!(!out_one.contains(&"sheen".to_string()));

        // Now a pattern proving >=2 E's: guess "eerie" against "steel".
        // steel = s,t,e,e,l. Expect at least one green + one yellow E.
        let pat_two_e = feedback(guess, "steel");
        let out_two = filter_by_guesses(
            &all,
            &[(guess.to_string(), pat_two_e)],
            RankMethod::Positional,
        );
        // "steel" survives its own pattern; single-E "ebony" cannot.
        assert!(out_two.contains(&"steel".to_string()));
        assert!(!out_two.contains(&"ebony".to_string()));
    }

    #[test]
    fn filter_by_guesses_multiple_guesses_intersect() {
        let all = words(&["crane", "slate", "trace", "plate", "grace", "brace"]);
        let answer = "grace";
        let g1 = ("crane".to_string(), feedback("crane", answer));
        let g2 = ("trace".to_string(), feedback("trace", answer));
        let out = filter_by_guesses(&all, &[g1, g2], RankMethod::Positional);
        // The true answer always survives the conjunction of its own feedbacks.
        assert!(out.contains(&"grace".to_string()));
        // Every survivor is consistent with both guesses.
        for w in &out {
            assert_eq!(feedback("crane", w), feedback("crane", answer));
            assert_eq!(feedback("trace", w), feedback("trace", answer));
        }
    }
}
