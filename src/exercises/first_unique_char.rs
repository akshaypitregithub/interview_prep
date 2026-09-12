//! G011: Find the first Unicode scalar value occurring exactly once.

use std::collections::HashMap;

/// Return the earliest char in input whose total frequency is exactly one.
/// Return None for empty input or if every char repeats. Case is significant.
/// Count whitespace, punctuation, combining marks and joiners without normalization.
/// Borrow input. Use safe Rust and explicit loops; no intermediate character vector,
/// sorting, or full-input scan per candidate. Multiple linear passes are allowed.
/// Aim for expected O(n) time and O(k) auxiliary space for k distinct chars.
/// Implement this exercise locally rather than calling an earlier exercise.
pub fn first_unique_char(input: &str) -> Option<char> {
    let mut counts: HashMap<char, bool> = HashMap::new();

    for char in input.chars() {
        match counts.get(&char) {
            Some(_v) => {
                counts.insert(char, false);
            }
            None => {
                counts.insert(char, true);
            }
        }
    }
    for char in input.chars() {
        if let Some(v) = counts.get(&char) {
            if *v {
                return Some(char);
            }
        } else {
            continue;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::first_unique_char;

    #[test]
    fn empty_input() {
        assert_eq!(first_unique_char(""), None);
    }
    #[test]
    fn single_character() {
        assert_eq!(first_unique_char("x"), Some('x'));
    }
    #[test]
    fn all_same() {
        assert_eq!(first_unique_char("aaaa"), None);
    }
    #[test]
    fn all_repeated_interleaved() {
        assert_eq!(first_unique_char("abcabc"), None);
    }
    #[test]
    fn first_in_input_order_not_sorted_order() {
        assert_eq!(first_unique_char("zabc"), Some('z'));
    }
    #[test]
    fn unique_in_middle() {
        assert_eq!(first_unique_char("aabcc"), Some('b'));
    }
    #[test]
    fn unique_at_end() {
        assert_eq!(first_unique_char("aabbz"), Some('z'));
    }
    #[test]
    fn later_repetition_disqualifies_early_candidate() {
        assert_eq!(first_unique_char("abac"), Some('b'));
    }
    #[test]
    fn case_is_significant() {
        assert_eq!(first_unique_char("aAa"), Some('A'));
    }
    #[test]
    fn whitespace_is_significant() {
        assert_eq!(first_unique_char("aa bb"), Some(' '));
    }
    #[test]
    fn punctuation_and_digits_are_significant() {
        assert_eq!(first_unique_char("11!22?"), Some('!'));
        assert_eq!(first_unique_char("aa1bb"), Some('1'));
    }
    #[test]
    fn controls_are_significant() {
        assert_eq!(first_unique_char("a\0a"), Some('\0'));
    }
    #[test]
    fn multibyte_characters_and_emoji() {
        assert_eq!(
            first_unique_char("\u{e9}\u{754c}\u{e9}\u{1f600}"),
            Some('\u{754c}')
        );
        assert_eq!(
            first_unique_char("\u{e9}\u{1f600}\u{e9}"),
            Some('\u{1f600}')
        );
    }
    #[test]
    fn combining_marks_are_independent_scalars() {
        assert_eq!(first_unique_char("e\u{301}e"), Some('\u{301}'));
        assert_eq!(first_unique_char("\u{e9}e\u{301}"), Some('\u{e9}'));
    }
    #[test]
    fn joiners_are_independent_scalars() {
        assert_eq!(
            first_unique_char("\u{1f469}\u{200d}\u{1f469}"),
            Some('\u{200d}')
        );
    }
    #[test]
    fn only_checks_supplied_slice() {
        assert_eq!(first_unique_char(&"xxaabccy"[2..7]), Some('b'));
    }
}
