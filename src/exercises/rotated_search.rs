//! G020: Search a rotated sorted slice with distinct values.

/// Input is a rotation of a strictly increasing i32 sequence (possibly empty
/// or unrotated). Values are distinct; do not validate or sort the input.
/// Return the target's index in the supplied slice, or None when absent.
/// Preserve input and support all i32 values and targets.
/// Use safe Rust and explicit loops: O(log n) time for nonempty input,
/// O(1) auxiliary space. No linear scan, sorting, recursion, built-in
/// search/partition helpers, or calls to earlier exercises.
pub fn rotated_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let mut left_ptr = 0;
    let mut right_ptr = nums.len() - 1;
    while left_ptr <= right_ptr {
        let mid_point = left_ptr + (right_ptr - left_ptr) / 2;
        if nums[mid_point] == target {
            return Some(mid_point);
        }
        let mut is_left_sorted = false;
        if nums[mid_point] >= nums[left_ptr] {
            is_left_sorted = true;
        }
        if is_left_sorted {
            if nums[left_ptr] <= target && nums[mid_point] > target {
                let right_ptr_check = mid_point.checked_sub(1);
                if let Some(right_ptr_unwrap) = right_ptr_check {
                    right_ptr = right_ptr_unwrap
                } else {
                    return None;
                }
            } else {
                left_ptr = mid_point + 1
            }
        } else {
            if nums[right_ptr] >= target && nums[mid_point] < target {
                left_ptr = mid_point + 1
            } else {
                let right_ptr_check = mid_point.checked_sub(1);
                if let Some(right_ptr_unwrap) = right_ptr_check {
                    right_ptr = right_ptr_unwrap
                } else {
                    return None;
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::rotated_search;

    #[test]
    fn empty_input() {
        for target in [i32::MIN, 0, i32::MAX] {
            assert_eq!(rotated_search(&[], target), None);
        }
    }

    #[test]
    fn singleton_present() {
        assert_eq!(rotated_search(&[7], 7), Some(0));
    }

    #[test]
    fn singleton_absent_on_either_side() {
        for target in [6, 8] {
            assert_eq!(rotated_search(&[7], target), None);
        }
    }

    #[test]
    fn two_elements_unrotated() {
        for (target, expected) in [(1, None), (2, Some(0)), (3, None), (5, Some(1)), (6, None)] {
            assert_eq!(rotated_search(&[2, 5], target), expected);
        }
    }

    #[test]
    fn two_elements_rotated() {
        for (target, expected) in [(1, None), (2, Some(1)), (3, None), (5, Some(0)), (6, None)] {
            assert_eq!(rotated_search(&[5, 2], target), expected);
        }
    }

    #[test]
    fn unrotated_odd_length_all_positions() {
        for (index, target) in [1, 3, 5, 7, 9].into_iter().enumerate() {
            assert_eq!(rotated_search(&[1, 3, 5, 7, 9], target), Some(index));
        }
    }

    #[test]
    fn unrotated_even_length_all_positions() {
        for (index, target) in [1, 3, 5, 7, 9, 11].into_iter().enumerate() {
            assert_eq!(rotated_search(&[1, 3, 5, 7, 9, 11], target), Some(index));
        }
    }

    #[test]
    fn finds_targets_on_both_sides_of_rotation() {
        let nums = [8, 10, 12, 2, 4, 6];
        for (index, &target) in nums.iter().enumerate() {
            assert_eq!(rotated_search(&nums, target), Some(index));
        }
    }

    #[test]
    fn minimum_at_index_one() {
        let nums = [11, 1, 3, 5, 7, 9];
        for (index, &target) in nums.iter().enumerate() {
            assert_eq!(rotated_search(&nums, target), Some(index));
        }
    }

    #[test]
    fn minimum_at_final_index() {
        let nums = [3, 5, 7, 9, 11, 1];
        for (index, &target) in nums.iter().enumerate() {
            assert_eq!(rotated_search(&nums, target), Some(index));
        }
    }

    #[test]
    fn missing_targets_in_both_sorted_runs() {
        for target in [3, 5, 9, 11] {
            assert_eq!(rotated_search(&[8, 10, 12, 2, 4, 6], target), None);
        }
    }

    #[test]
    fn absent_below_above_and_between_runs() {
        for target in [0, 7, 13] {
            assert_eq!(rotated_search(&[8, 10, 12, 2, 4, 6], target), None);
        }
    }

    #[test]
    fn negatives_and_zero() {
        let nums = [0, 4, 9, -8, -3];
        for (index, &target) in nums.iter().enumerate() {
            assert_eq!(rotated_search(&nums, target), Some(index));
        }
        assert_eq!(rotated_search(&nums, -1), None);
    }

    #[test]
    fn integer_extremes() {
        let nums = [0, i32::MAX, i32::MIN, -1];
        for (index, &target) in nums.iter().enumerate() {
            assert_eq!(rotated_search(&nums, target), Some(index));
        }
        for target in [i32::MIN + 1, i32::MAX - 1] {
            assert_eq!(rotated_search(&nums, target), None);
        }
        assert_eq!(rotated_search(&[i32::MIN], i32::MAX), None);
        assert_eq!(rotated_search(&[i32::MAX], i32::MIN), None);
    }

    #[test]
    fn supplied_slice_indices_and_input_preserved() {
        let nums = [99, 8, 10, 2, 4, -99];
        assert_eq!(rotated_search(&nums[1..5], 2), Some(2));
        assert_eq!(rotated_search(&nums[1..5], 99), None);
        assert_eq!(nums, [99, 8, 10, 2, 4, -99]);
    }

    #[test]
    fn every_rotation_of_small_inputs_matches_oracle() {
        for len in 1..=16 {
            let sorted: Vec<i32> = (0..len).map(|x| x * 2 - 16).collect();
            for shift in 0..sorted.len() {
                let mut nums = sorted.clone();
                nums.rotate_left(shift);
                for target in -17..=16 {
                    let expected = nums.iter().position(|&value| value == target);
                    assert_eq!(
                        rotated_search(&nums, target),
                        expected,
                        "nums={nums:?}, target={target}"
                    );
                }
            }
        }
    }

    #[test]
    fn larger_rotated_input_positions_and_gaps() {
        let mut nums: Vec<i32> = (0..1025).map(|x| x * 2).collect();
        nums.rotate_left(513);
        for target in -1..=2049 {
            let expected = nums.iter().position(|&value| value == target);
            assert_eq!(rotated_search(&nums, target), expected, "target={target}");
        }
    }
}
