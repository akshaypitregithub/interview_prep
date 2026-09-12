//! G001: Find the first matching index. See CURRENT_GOAL.md.
/// Return the first zero-based index of `target`, or `None` when absent.
///
/// The input may be empty, unsorted, or contain duplicates and any `i32` value.
/// Indices are relative to the supplied slice. Do not copy or mutate the input.
/// For this exercise, use a loop rather than a built-in search helper.
/// Target: O(n) worst-case time and O(1) auxiliary space.
pub fn first_index(nums: &[i32], target: i32) -> Option<usize> {
    for (i, num) in nums.iter().enumerate() {
        if *num == target {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::first_index;

    #[test]
    fn empty_input_returns_none() {
        assert_eq!(first_index(&[], 7), None);
    }

    #[test]
    fn singleton_match_returns_zero() {
        assert_eq!(first_index(&[7], 7), Some(0));
    }

    #[test]
    fn singleton_miss_returns_none() {
        assert_eq!(first_index(&[7], 3), None);
    }

    #[test]
    fn finds_first_element() {
        assert_eq!(first_index(&[9, 4, 6], 9), Some(0));
    }

    #[test]
    fn finds_middle_element_in_unsorted_input() {
        assert_eq!(first_index(&[9, 4, 6, 1, 8], 6), Some(2));
    }

    #[test]
    fn finds_last_element() {
        assert_eq!(first_index(&[9, 4, 6], 6), Some(2));
    }

    #[test]
    fn absent_target_returns_none() {
        assert_eq!(first_index(&[9, 4, 6], 5), None);
    }

    #[test]
    fn duplicate_matches_return_the_first_index() {
        assert_eq!(first_index(&[8, 3, 8, 3], 3), Some(1));
    }

    #[test]
    fn all_equal_values_return_zero_or_none() {
        assert_eq!(first_index(&[5, 5, 5, 5], 5), Some(0));
        assert_eq!(first_index(&[5, 5, 5, 5], 4), None);
    }

    #[test]
    fn handles_negative_values_and_zero() {
        let nums = [-3, 0, -8, 2];
        assert_eq!(first_index(&nums, -8), Some(2));
        assert_eq!(first_index(&nums, 0), Some(1));
        assert_eq!(first_index(&nums, -1), None);
    }

    #[test]
    fn handles_integer_limits_without_arithmetic_overflow() {
        let nums = [0, i32::MAX, i32::MIN, i32::MAX];
        assert_eq!(first_index(&nums, i32::MIN), Some(2));
        assert_eq!(first_index(&nums, i32::MAX), Some(1));
        assert_eq!(first_index(&nums, -1), None);
    }

    #[test]
    fn returns_an_index_relative_to_the_borrowed_subslice() {
        let nums = [99, 8, 3, 8, 77];
        assert_eq!(first_index(&nums[1..4], 8), Some(0));
        assert_eq!(first_index(&nums[1..4], 3), Some(1));
        assert_eq!(first_index(&nums[1..4], 99), None);
    }
}
