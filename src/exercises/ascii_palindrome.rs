//! G009: Check a case-insensitive ASCII alphanumeric palindrome.

/// Return None if any input character is non-ASCII, even punctuation.
/// Otherwise return Some(true) if the ASCII letters/digits form a palindrome,
/// ignoring letter case and all other ASCII bytes; return Some(false) if not.
/// Empty input or no retained letters/digits is a palindrome.
/// Borrow input; do not allocate a normalized string/vector or reverse a copy.
/// Use explicit loops and safe Rust. ASCII classification/conversion helpers
/// and an ASCII-validation helper are allowed. Aim for O(n) time, O(1) space.
pub fn ascii_palindrome(input: &str) -> Option<bool> {
    if input.is_empty() {
        return Some(true);
    }
    if !input.is_ascii() {
        return None;
    }
    let mut left_ptr = 0;
    let mut right_ptr = input.len() - 1;

    loop {
        if left_ptr >= right_ptr {
            break;
        }
        let left_char = input.as_bytes()[left_ptr];
        if !left_char.is_ascii_alphanumeric() {
            left_ptr += 1;
            continue;
        }
        let right_char = input.as_bytes()[right_ptr];
        if !right_char.is_ascii_alphanumeric() {
            right_ptr -= 1;
            continue;
        }
        if !right_char.eq_ignore_ascii_case(&left_char) {
            return Some(false);
        } else {
            left_ptr += 1;
            right_ptr -= 1;
        }
    }

    Some(true)
}

#[cfg(test)]
mod tests {
    use super::ascii_palindrome;

    #[test]
    fn empty_input_is_a_palindrome() {
        assert_eq!(ascii_palindrome(""), Some(true));
    }

    #[test]
    fn one_letter_or_digit_is_a_palindrome() {
        assert_eq!(ascii_palindrome("A"), Some(true));
        assert_eq!(ascii_palindrome("7"), Some(true));
    }

    #[test]
    fn ignores_ascii_case() {
        assert_eq!(ascii_palindrome("RaceCar"), Some(true));
        assert_eq!(ascii_palindrome("aBba"), Some(true));
    }

    #[test]
    fn detects_odd_and_even_non_palindromes() {
        assert_eq!(ascii_palindrome("abc"), Some(false));
        assert_eq!(ascii_palindrome("abca"), Some(false));
        assert_eq!(ascii_palindrome("ab"), Some(false));
    }

    #[test]
    fn ignores_spaces_and_punctuation() {
        assert_eq!(
            ascii_palindrome("A man, a plan, a canal: Panama!"),
            Some(true)
        );
        assert_eq!(ascii_palindrome("race a car"), Some(false));
    }

    #[test]
    fn no_alphanumeric_bytes_is_a_palindrome() {
        assert_eq!(ascii_palindrome(" !?_-\t\n"), Some(true));
    }

    #[test]
    fn handles_ignored_bytes_at_either_end_and_in_middle() {
        assert_eq!(ascii_palindrome("!!!a"), Some(true));
        assert_eq!(ascii_palindrome("a???"), Some(true));
        assert_eq!(ascii_palindrome("--a...A!!!"), Some(true));
        assert_eq!(ascii_palindrome("--a...b!!!"), Some(false));
    }

    #[test]
    fn digits_are_significant() {
        assert_eq!(ascii_palindrome("12-21"), Some(true));
        assert_eq!(ascii_palindrome("12-31"), Some(false));
        assert_eq!(ascii_palindrome("0P"), Some(false));
    }

    #[test]
    fn letters_and_digits_can_mix() {
        assert_eq!(ascii_palindrome("A1,1a"), Some(true));
        assert_eq!(ascii_palindrome("A1,2a"), Some(false));
    }

    #[test]
    fn ascii_controls_are_ignored() {
        assert_eq!(ascii_palindrome("\0A\n\t a\u{7f}"), Some(true));
    }

    #[test]
    fn rejects_non_ascii_letters_digits_and_symbols() {
        for input in ["\u{e9}", "\u{ff11}", "\u{1f600}", "a\u{2014}a"] {
            assert_eq!(ascii_palindrome(input), None, "input: {input:?}");
        }
    }

    #[test]
    fn non_ascii_validation_takes_precedence_over_mismatch() {
        assert_eq!(ascii_palindrome("ab\u{e9}cd"), None);
        assert_eq!(ascii_palindrome("\u{e9}ab"), None);
        assert_eq!(ascii_palindrome("ab\u{e9}"), None);
    }

    #[test]
    fn only_checks_the_supplied_string_slice() {
        let text = "xRaceCary";
        assert_eq!(ascii_palindrome(&text[1..8]), Some(true));
        assert_eq!(ascii_palindrome(text), Some(false));
    }
}
