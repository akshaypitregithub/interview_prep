//! G010: Count Unicode scalar values.
use std::collections::HashMap;

/// Count every Rust char, including whitespace, punctuation, and combining marks.
/// Case is significant; do not normalize Unicode or group grapheme clusters.
/// Borrow input, use an explicit loop, and avoid an intermediate character vector.
/// Aim for expected O(n) time and O(k) output space for k distinct characters.
pub fn character_frequencies(input: &str) -> HashMap<char, usize> {
    let _ = input;
    let mut map: HashMap<char, usize> = HashMap::new();
    for char in input.chars() {
        match map.get(&char) {
            Some(v) => {
                let new_v = *v + 1;
                map.insert(char, new_v);
            }
            None => {
                map.insert(char, 1);
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::character_frequencies;
    use std::collections::HashMap;
    fn check(input: &str, expected: &[(char, usize)]) {
        assert_eq!(
            character_frequencies(input),
            HashMap::from_iter(expected.iter().copied())
        );
    }
    #[test]
    fn empty_input() {
        check("", &[]);
    }
    #[test]
    fn single_character() {
        check("z", &[('z', 1)]);
    }
    #[test]
    fn repeated_character() {
        check("aaaaa", &[('a', 5)]);
    }
    #[test]
    fn counts_interleaved_characters() {
        check("banana", &[('b', 1), ('a', 3), ('n', 2)]);
    }
    #[test]
    fn distinct_characters() {
        check("rust", &[('r', 1), ('u', 1), ('s', 1), ('t', 1)]);
    }
    #[test]
    fn case_is_significant() {
        check("aAaB", &[('a', 2), ('A', 1), ('B', 1)]);
    }
    #[test]
    fn punctuation_and_digits() {
        check("1!1?_!", &[('1', 2), ('!', 2), ('?', 1), ('_', 1)]);
    }
    #[test]
    fn whitespace_and_controls() {
        check(" \t\n \0", &[(' ', 2), ('\t', 1), ('\n', 1), ('\0', 1)]);
    }
    #[test]
    fn multibyte_letters() {
        check("\u{e9}\u{754c}\u{e9}", &[('\u{e9}', 2), ('\u{754c}', 1)]);
    }
    #[test]
    fn emoji() {
        check("\u{1f600}a\u{1f600}", &[('\u{1f600}', 2), ('a', 1)]);
    }
    #[test]
    fn no_unicode_normalization() {
        check(
            "\u{e9}e\u{301}e\u{301}",
            &[('\u{e9}', 1), ('e', 2), ('\u{301}', 2)],
        );
    }
    #[test]
    fn joined_emoji_are_multiple_scalars() {
        check(
            "\u{1f469}\u{200d}\u{1f4bb}",
            &[('\u{1f469}', 1), ('\u{200d}', 1), ('\u{1f4bb}', 1)],
        );
    }
    #[test]
    fn supplied_slice_only() {
        check(&"xbananay"[1..7], &[('b', 1), ('a', 3), ('n', 2)]);
    }
}
