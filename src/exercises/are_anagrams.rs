//! G012: Compare Unicode scalar-value multiplicities.

use std::collections::HashMap;

/// Return true exactly when both strings contain the same chars with the same counts.
/// Order is irrelevant; case, whitespace, punctuation and combining marks matter.
/// Do not normalize Unicode or group grapheme clusters. Empty strings are anagrams.
/// Borrow both inputs. Use safe Rust, explicit loops and standard library collections.
/// No sorting, intermediate character vectors, or full-input scan per character.
/// Implement locally. Aim for expected O(n + m) time and O(k) auxiliary space,
/// where n and m are input byte lengths and k is the number of distinct chars overall.
pub fn are_anagrams(left: &str, right: &str) -> bool {
    if left.len() != right.len() {
        return false;
    }

    pub struct Anagram {
        left: usize,
        right: usize,
    }

    let mut anagram_map: HashMap<char, Anagram> = HashMap::new();

    let mut left_chars = left.chars();
    let mut right_chars = right.chars();

    loop {
        let left_char_option = left_chars.next();
        if left_char_option.is_none() {
            break;
        }
        let right_char_option = right_chars.next();
        if right_char_option.is_none() {
            break;
        }
        anagram_map
            .entry(left_char_option.unwrap())
            .and_modify(|anagram| {
                anagram.left += 1;
            })
            .or_insert(Anagram { left: 1, right: 0 });
        anagram_map
            .entry(right_char_option.unwrap())
            .and_modify(|anagram| {
                anagram.right += 1;
            })
            .or_insert(Anagram { left: 0, right: 1 });
    }
    for (_, value) in anagram_map {
        if value.right != value.left {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::are_anagrams;
    #[test]
    fn both_empty() {
        assert!(are_anagrams("", ""));
    }
    #[test]
    fn only_one_empty() {
        assert!(!are_anagrams("a", ""));
        assert!(!are_anagrams("", "a"));
    }
    #[test]
    fn single_characters() {
        assert!(are_anagrams("a", "a"));
        assert!(!are_anagrams("a", "b"));
    }
    #[test]
    fn reordered_letters() {
        assert!(are_anagrams("listen", "silent"));
        assert!(are_anagrams("silent", "listen"));
    }
    #[test]
    fn matching_repeated_counts() {
        assert!(are_anagrams("aabbbc", "babcba"));
    }
    #[test]
    fn same_keys_different_counts() {
        assert!(!are_anagrams("aab", "abb"));
    }
    #[test]
    fn odd_repetitions_are_not_unique() {
        assert!(!are_anagrams("aaab", "abbb"));
    }
    #[test]
    fn different_lengths() {
        assert!(!are_anagrams("ab", "aab"));
        assert!(!are_anagrams("aab", "ab"));
    }
    #[test]
    fn equal_length_different_keys() {
        assert!(!are_anagrams("abc", "abd"));
    }
    #[test]
    fn case_is_significant() {
        assert!(are_anagrams("aA", "Aa"));
        assert!(!are_anagrams("aA", "aa"));
    }
    #[test]
    fn whitespace_and_punctuation_are_significant() {
        assert!(are_anagrams("a !", "! a"));
        assert!(!are_anagrams("a!", "a?"));
        assert!(!are_anagrams("a ", "a\t"));
    }
    #[test]
    fn digits_and_controls() {
        assert!(are_anagrams("1\0\n1", "\n11\0"));
    }
    #[test]
    fn multibyte_letters_and_emoji() {
        assert!(are_anagrams(
            "\u{e9}\u{754c}\u{1f600}\u{e9}",
            "\u{1f600}\u{e9}\u{e9}\u{754c}"
        ));
    }
    #[test]
    fn byte_multisets_are_not_character_multisets() {
        assert!(!are_anagrams("\u{a1}\u{e2}", "\u{a2}\u{e1}"));
    }
    #[test]
    fn does_not_normalize_unicode() {
        assert!(!are_anagrams("\u{e9}", "e\u{301}"));
    }
    #[test]
    fn combining_marks_and_joiners_are_independent_scalars() {
        assert!(are_anagrams("e\u{301}", "\u{301}e"));
        assert!(are_anagrams(
            "\u{1f469}\u{200d}\u{1f4bb}",
            "\u{1f4bb}\u{200d}\u{1f469}"
        ));
    }
    #[test]
    fn supplied_slices_only() {
        assert!(are_anagrams(&"xlisteny"[1..7], &"zsilentw"[1..7]));
    }
}
