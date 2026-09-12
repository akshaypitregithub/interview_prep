//! G018: Binary search in a sorted slice.

/// Return any index containing target, or None if absent.
/// Input is guaranteed sorted in nondecreasing order; duplicates are allowed.
/// Indices are relative to the supplied slice. Preserve input.
/// Use safe Rust and an explicit loop. No built-in search/partition helper,
/// linear scan, sorting, recursion, or calls to earlier exercises.
/// Aim for O(log n) time and O(1) auxiliary space.
pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let mut left_ptr: usize = 0;
    let mut right_ptr: usize = nums.len() - 1;
    loop {
        let mid_index = left_ptr + (right_ptr - left_ptr) / 2;
        if nums[mid_index] == target {
            return Some(mid_index);
        }
        if right_ptr == left_ptr {
            return None;
        }
        if target > nums[mid_index] {
            left_ptr = mid_index + 1
        } else {
            right_ptr = mid_index
        }
    }
}

#[cfg(test)]
mod tests {
    use super::binary_search;
    fn check(nums: &[i32], target: i32, present: bool) {
        match binary_search(nums, target) {
            Some(index) => {
                assert!(present, "unexpected index {index}");
                assert!(index < nums.len(), "out-of-bounds index {index}");
                assert_eq!(nums[index], target);
            }
            None => assert!(!present, "missed target {target} in {nums:?}"),
        }
    }
    #[test]
    fn empty_input() {
        check(&[], 0, false);
    }
    #[test]
    fn singleton_present() {
        check(&[7], 7, true);
    }
    #[test]
    fn singleton_absent_on_either_side() {
        check(&[7], 6, false);
        check(&[7], 8, false);
    }
    #[test]
    fn two_elements_both_searchable() {
        check(&[2, 5], 2, true);
        check(&[2, 5], 5, true);
    }
    #[test]
    fn two_elements_absent_between() {
        check(&[2, 5], 3, false);
    }
    #[test]
    fn odd_length_middle() {
        check(&[1, 3, 5, 7, 9], 5, true);
    }
    #[test]
    fn even_length_interior() {
        check(&[1, 3, 5, 7, 9, 11], 5, true);
        check(&[1, 3, 5, 7, 9, 11], 7, true);
    }
    #[test]
    fn first_and_last() {
        check(&[-5, -1, 0, 3, 8], -5, true);
        check(&[-5, -1, 0, 3, 8], 8, true);
    }
    #[test]
    fn absent_below_and_above() {
        check(&[1, 3, 5], 0, false);
        check(&[1, 3, 5], 6, false);
    }
    #[test]
    fn absent_interior() {
        check(&[1, 3, 5, 7, 9], 6, false);
    }
    #[test]
    fn any_duplicate_index_is_accepted() {
        check(&[1, 2, 2, 2, 3], 2, true);
    }
    #[test]
    fn all_equal() {
        check(&[4, 4, 4, 4], 4, true);
        check(&[4, 4, 4, 4], 3, false);
        check(&[4, 4, 4, 4], 5, false);
    }
    #[test]
    fn negatives_and_zero() {
        check(&[-9, -6, -2, 0, 4], -6, true);
        check(&[-9, -6, -2, 0, 4], 0, true);
    }
    #[test]
    fn integer_extremes() {
        check(&[i32::MIN, 0, i32::MAX], i32::MIN, true);
        check(&[i32::MIN, 0, i32::MAX], i32::MAX, true);
        check(&[i32::MIN, i32::MAX], 0, false);
    }
    #[test]
    fn supplied_slice_indices() {
        let nums = [-99, 2, 4, 6, 99];
        assert_eq!(binary_search(&nums[1..4], 6), Some(2));
        assert_eq!(nums, [-99, 2, 4, 6, 99]);
    }
    #[test]
    fn all_positions_and_gaps_in_larger_input() {
        let nums: Vec<i32> = (0..257).map(|x| x * 2).collect();
        for target in 0..514 {
            check(&nums, target, target % 2 == 0);
        }
    }
}
