//! G013: Find two distinct indices whose values sum to a target.

use std::collections::HashMap;

/// Return any pair (i, j) with i < j whose mathematical sum equals target.
/// Return None when no pair exists. Never reuse an index. Preserve input.
/// Use safe Rust and an explicit loop; standard library collections are allowed.
/// No sorting, all-pairs search, or calls to earlier exercises.
/// Arithmetic must not panic or treat wrapped sums as matches.
/// Aim for expected O(n) time and O(n) auxiliary space.
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut remainder_map: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let seen_indices = target.checked_sub(*num);
        if let Some(remainder) = seen_indices {
            if let Some(cached_idx) = remainder_map.get(&remainder) {
                return Some((*cached_idx, i));
            } else {
                remainder_map.insert(*num, i);
            }
        } else {
            continue;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    fn check(nums: &[i32], target: i32, has_pair: bool) {
        let original = nums.to_vec();
        let result = two_sum(nums, target);
        assert_eq!(nums, original.as_slice());
        match result {
            Some((i, j)) => {
                assert!(has_pair, "unexpected pair: {result:?}");
                assert!(i < j && j < nums.len(), "invalid indices: {result:?}");
                assert_eq!(i64::from(nums[i]) + i64::from(nums[j]), i64::from(target));
            }
            None => assert!(!has_pair, "expected a pair for {nums:?}, target {target}"),
        }
    }

    #[test]
    fn empty_input() {
        check(&[], 0, false);
    }
    #[test]
    fn cannot_reuse_single_index() {
        check(&[3], 6, false);
    }
    #[test]
    fn basic_pair() {
        check(&[2, 7, 11, 15], 9, true);
    }
    #[test]
    fn pair_away_from_start() {
        check(&[8, 3, 2, 4], 6, true);
    }
    #[test]
    fn no_pair() {
        check(&[1, 2, 4], 8, false);
    }
    #[test]
    fn duplicate_values_at_distinct_indices() {
        check(&[3, 3], 6, true);
    }
    #[test]
    fn one_occurrence_of_half_target_is_insufficient() {
        check(&[3, 1, 8], 6, false);
    }
    #[test]
    fn zero_pairs() {
        check(&[0, 0], 0, true);
        check(&[0], 0, false);
    }
    #[test]
    fn negatives() {
        check(&[-5, -2, -8, -3], -10, true);
    }
    #[test]
    fn mixed_signs() {
        check(&[-3, 9, 3, 5], 0, true);
    }
    #[test]
    fn any_valid_pair_is_accepted() {
        check(&[1, 4, 2, 3, 0, 5], 5, true);
    }
    #[test]
    fn extremes_can_form_valid_sum() {
        check(&[i32::MIN, i32::MAX], -1, true);
    }
    #[test]
    fn out_of_range_complement_does_not_stop_search() {
        check(&[i32::MIN, 0, 1], 1, true);
        check(&[i32::MAX, 0, -1], -1, true);
    }
    #[test]
    fn wrapped_sums_are_not_matches() {
        check(&[i32::MAX, 1], i32::MIN, false);
        check(&[i32::MIN, -1], i32::MAX, false);
    }
    #[test]
    fn boundary_targets() {
        check(&[i32::MIN, 0], i32::MIN, true);
        check(&[0, i32::MAX], i32::MAX, true);
    }
    #[test]
    fn indices_are_relative_to_supplied_slice() {
        let nums = [99, 2, 7, 88];
        assert_eq!(two_sum(&nums[1..3], 9), Some((0, 1)));
    }
}
