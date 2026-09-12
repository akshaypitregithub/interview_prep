//! G002: Count every occurrence of a target in a borrowed slice.

/// Return how many elements equal `target`, including repeated matches.
/// Empty input or an absent target returns zero. All `i32` values are valid.
/// Do not mutate or copy the slice. Use an explicit loop for this exercise,
/// rather than iterator counting helpers. Aim for O(n) time and O(1) extra space.
pub fn count_occurrences(nums: &[i32], target: i32) -> usize {
    let mut count = 0;
    for num in nums {
        if *num == target {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::count_occurrences;

    #[test]
    fn empty_input_returns_zero() {
        assert_eq!(count_occurrences(&[], 7), 0);
    }

    #[test]
    fn singleton_match_returns_one() {
        assert_eq!(count_occurrences(&[7], 7), 1);
    }

    #[test]
    fn singleton_miss_returns_zero() {
        assert_eq!(count_occurrences(&[7], 3), 0);
    }

    #[test]
    fn absent_target_returns_zero() {
        assert_eq!(count_occurrences(&[9, 4, 6], 5), 0);
    }

    #[test]
    fn counts_a_single_match_at_each_position() {
        let nums = [9, 4, 6];
        assert_eq!(count_occurrences(&nums, 9), 1);
        assert_eq!(count_occurrences(&nums, 4), 1);
        assert_eq!(count_occurrences(&nums, 6), 1);
    }

    #[test]
    fn counts_nonadjacent_matches_including_both_ends() {
        assert_eq!(count_occurrences(&[8, 3, 8, 4, 8], 8), 3);
    }

    #[test]
    fn counts_adjacent_matches() {
        assert_eq!(count_occurrences(&[9, 2, 2, 2, 7], 2), 3);
    }

    #[test]
    fn all_equal_values_return_length_or_zero() {
        assert_eq!(count_occurrences(&[5, 5, 5, 5], 5), 4);
        assert_eq!(count_occurrences(&[5, 5, 5, 5], 4), 0);
    }

    #[test]
    fn counts_negative_values_and_zero() {
        let nums = [-3, 0, -3, 0, -3, 2];
        assert_eq!(count_occurrences(&nums, -3), 3);
        assert_eq!(count_occurrences(&nums, 0), 2);
    }

    #[test]
    fn handles_integer_limits() {
        let nums = [i32::MIN, i32::MAX, i32::MIN, 0];
        assert_eq!(count_occurrences(&nums, i32::MIN), 2);
        assert_eq!(count_occurrences(&nums, i32::MAX), 1);
        assert_eq!(count_occurrences(&nums, -1), 0);
    }

    #[test]
    fn only_counts_elements_in_the_supplied_subslice() {
        let nums = [8, 8, 3, 8, 8];
        assert_eq!(count_occurrences(&nums[1..4], 8), 2);
        assert_eq!(count_occurrences(&nums[2..2], 8), 0);
    }
}
