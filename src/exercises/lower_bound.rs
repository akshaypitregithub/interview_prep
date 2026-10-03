//! G019: Find the first position whose value is at least target.

/// Input is guaranteed sorted in nondecreasing order; duplicates are allowed.
/// Return the smallest index i with nums[i] >= target, or nums.len() if none.
/// Empty input returns 0. Indices are relative to the supplied slice.
/// Preserve input; support all i32 values and targets.
/// Use safe Rust and an explicit loop, with O(log n) time for nonempty input
/// and O(1) auxiliary space. No search/partition helpers, linear scan, sorting,
/// recursion, or calls to earlier exercises.
pub fn lower_bound(nums: &[i32], target: i32) -> usize {
    if nums.is_empty() {
        return nums.len();
    }
    let mut left_ptr = 0;
    let mut right_ptr = nums.len() - 1;

    loop {
        let mid_index = left_ptr + (right_ptr - left_ptr) / 2;

        let mid_num = nums[mid_index];

        if right_ptr - left_ptr <= 1 {
            if nums[left_ptr] >= target {
                return left_ptr;
            }
            if nums[right_ptr] < target {
                return nums.len();
            }
            if nums[left_ptr] < target && nums[right_ptr] >= target {
                return right_ptr;
            }
        }

        if target <= mid_num {
            right_ptr = mid_index;
        } else {
            left_ptr = mid_index;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lower_bound;

    #[test]
    fn empty_input() {
        for target in [i32::MIN, 0, i32::MAX] {
            assert_eq!(lower_bound(&[], target), 0);
        }
    }

    #[test]
    fn singleton_target_before_equal_and_after() {
        assert_eq!(lower_bound(&[7], 6), 0);
        assert_eq!(lower_bound(&[7], 7), 0);
        assert_eq!(lower_bound(&[7], 8), 1);
    }

    #[test]
    fn two_elements_all_positions_and_gaps() {
        for (target, expected) in [(1, 0), (2, 0), (3, 1), (5, 1), (6, 2)] {
            assert_eq!(lower_bound(&[2, 5], target), expected);
        }
    }

    #[test]
    fn two_equal_elements() {
        assert_eq!(lower_bound(&[4, 4], 4), 0);
        assert_eq!(lower_bound(&[4, 4], 5), 2);
    }

    #[test]
    fn exact_matches_in_odd_length_input() {
        for (index, target) in [1, 3, 5, 7, 9].into_iter().enumerate() {
            assert_eq!(lower_bound(&[1, 3, 5, 7, 9], target), index);
        }
    }

    #[test]
    fn exact_matches_in_even_length_input() {
        for (index, target) in [1, 3, 5, 7, 9, 11].into_iter().enumerate() {
            assert_eq!(lower_bound(&[1, 3, 5, 7, 9, 11], target), index);
        }
    }

    #[test]
    fn missing_interior_target_returns_next_position() {
        assert_eq!(lower_bound(&[1, 3, 7, 9], 5), 2);
    }

    #[test]
    fn target_below_all_values() {
        assert_eq!(lower_bound(&[2, 4, 6, 8], -1), 0);
    }

    #[test]
    fn target_above_all_values_returns_length() {
        assert_eq!(lower_bound(&[2, 4, 6, 8], 9), 4);
    }

    #[test]
    fn duplicates_in_middle_return_first() {
        assert_eq!(lower_bound(&[1, 2, 2, 2, 3], 2), 1);
    }

    #[test]
    fn duplicates_at_start_return_first() {
        assert_eq!(lower_bound(&[2, 2, 2, 4, 6], 2), 0);
    }

    #[test]
    fn duplicates_at_end_return_first() {
        assert_eq!(lower_bound(&[1, 3, 5, 5, 5], 5), 2);
    }

    #[test]
    fn all_equal_values() {
        for (target, expected) in [(3, 0), (4, 0), (5, 5)] {
            assert_eq!(lower_bound(&[4; 5], target), expected);
        }
    }

    #[test]
    fn negatives_and_zero() {
        let nums = [-9, -6, -6, -2, 0, 4];
        for (target, expected) in [(-6, 1), (-5, 3), (-1, 4), (0, 4), (1, 5)] {
            assert_eq!(lower_bound(&nums, target), expected);
        }
    }

    #[test]
    fn integer_extremes() {
        let nums = [i32::MIN, i32::MIN, 0, i32::MAX, i32::MAX];
        assert_eq!(lower_bound(&nums, i32::MIN), 0);
        assert_eq!(lower_bound(&nums, i32::MIN + 1), 2);
        assert_eq!(lower_bound(&nums, i32::MAX), 3);
        assert_eq!(lower_bound(&[i32::MIN], i32::MAX), 1);
        assert_eq!(lower_bound(&[i32::MAX], i32::MIN), 0);
    }

    #[test]
    fn supplied_slice_indices_and_input_preserved() {
        let nums = [-99, 2, 4, 4, 6, 99];
        assert_eq!(lower_bound(&nums[1..5], 4), 1);
        assert_eq!(lower_bound(&nums[1..5], 7), 4);
        assert_eq!(nums, [-99, 2, 4, 4, 6, 99]);
    }

    #[test]
    fn larger_duplicate_runs_and_gaps() {
        let nums: Vec<i32> = (0..257).flat_map(|x| [x * 2; 3]).collect();
        for target in -1..=514 {
            let expected = nums
                .iter()
                .position(|&value| value >= target)
                .unwrap_or(nums.len());
            assert_eq!(lower_bound(&nums, target), expected, "target {target}");
        }
    }
}
