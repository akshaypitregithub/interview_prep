//! G017: Longest contiguous substring with distinct Unicode scalar values.

use std::collections::HashSet;

/// Return the maximum number of chars in a contiguous substring with no repeated char.
/// Empty input returns zero. Count Unicode scalar values, not bytes or graphemes.
/// Case, whitespace, punctuation, combining marks and joiners are significant.
/// Borrow input; do not normalize it or collect an intermediate character vector.
/// Use safe Rust, explicit loops and standard library collections; no sorting,
/// earlier exercise calls or full rescans per candidate. Aim for expected O(n)
/// time for n input bytes and O(k) auxiliary space for k distinct input chars.
pub fn longest_unique_substring(input: &str) -> usize {
    let mut largest_contiguous_substring: usize = 0;
    let mut left_ptr: usize = 0;
    let mut right_ptr: usize = 0;
    let mut char_cache: HashSet<char> = HashSet::new();

    let mut right_iter = input.chars();

    let mut left_iter = input.chars();

    loop {
        let current_char_optional = right_iter.next();

        if let Some(current_char) = current_char_optional {
            if char_cache.contains(&current_char) {
                loop {
                    let left_current_char_option = left_iter.next();
                    if let Some(left_current_char) = left_current_char_option {
                        if current_char != left_current_char {
                            char_cache.remove(&left_current_char);
                            left_ptr += 1;
                        } else {
                            left_ptr += 1;
                            break;
                        }
                    }
                }
            } else {
                char_cache.insert(current_char);
            }
            right_ptr += 1;
            if right_ptr - left_ptr > largest_contiguous_substring {
                largest_contiguous_substring = right_ptr - left_ptr;
            }
        } else {
            break;
        }
    }

    largest_contiguous_substring
}

#[cfg(test)]
mod tests {
    use super::longest_unique_substring;
    #[test]
    fn empty_input() {
        assert_eq!(longest_unique_substring(""), 0);
    }
    #[test]
    fn singleton() {
        assert_eq!(longest_unique_substring("x"), 1);
    }
    #[test]
    fn all_identical() {
        assert_eq!(longest_unique_substring("aaaa"), 1);
    }
    #[test]
    fn all_distinct() {
        assert_eq!(longest_unique_substring("rust"), 4);
    }
    #[test]
    fn repeated_pattern() {
        assert_eq!(longest_unique_substring("abcabcbb"), 3);
    }
    #[test]
    fn must_be_contiguous() {
        assert_eq!(longest_unique_substring("pwwkew"), 3);
    }
    #[test]
    fn old_duplicate_must_not_move_start_backward() {
        assert_eq!(longest_unique_substring("abba"), 2);
    }
    #[test]
    fn repeated_char_does_not_require_discarding_whole_window() {
        assert_eq!(longest_unique_substring("dvdf"), 3);
    }
    #[test]
    fn maximum_at_end() {
        assert_eq!(longest_unique_substring("aabcde"), 5);
    }
    #[test]
    fn maximum_at_start() {
        assert_eq!(longest_unique_substring("abcdeee"), 5);
    }
    #[test]
    fn case_is_significant() {
        assert_eq!(longest_unique_substring("aAbA"), 3);
    }
    #[test]
    fn whitespace_punctuation_and_controls_count() {
        assert_eq!(longest_unique_substring("a !a"), 3);
        assert_eq!(longest_unique_substring("\0\t\n\0"), 3);
    }
    #[test]
    fn multibyte_characters_count_once() {
        assert_eq!(longest_unique_substring("\u{e9}\u{754c}\u{1f600}\u{e9}"), 3);
    }
    #[test]
    fn unicode_duplicate_outside_current_window() {
        assert_eq!(longest_unique_substring("\u{e9}\u{754c}\u{754c}\u{e9}"), 2);
    }
    #[test]
    fn no_normalization_or_grapheme_grouping() {
        assert_eq!(longest_unique_substring("\u{e9}e\u{301}"), 3);
        assert_eq!(longest_unique_substring("e\u{301}e"), 2);
    }
    #[test]
    fn emoji_joiner_counts_as_a_scalar() {
        assert_eq!(longest_unique_substring("\u{1f469}\u{200d}\u{1f4bb}"), 3);
    }
    #[test]
    fn supplied_slice_only() {
        assert_eq!(longest_unique_substring(&"zabbay"[1..5]), 2);
    }
}
