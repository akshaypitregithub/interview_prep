//! G007: Compact distinct values in a sorted mutable slice.

/// Input is sorted in nondecreasing order; unsorted input is outside the contract.
/// Write each distinct value once, in sorted order, into the first k slots.
/// Return k. Empty input returns zero. Elements after k are unspecified.
/// The physical slice length does not change. Do not allocate another collection.
/// Use an explicit loop and safe Rust, not built-in deduplication helpers.
/// Aim for O(n) time and O(1) auxiliary space.
pub fn dedup_sorted(nums: &mut [i32]) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let mut count = 1;
    let mut current_val = nums[0];
    for i in 0..nums.len() {
        if nums[i] > current_val {
            current_val = nums[i];
            nums[count] = current_val;
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::dedup_sorted;

    fn assert_compacted(input: &mut [i32], expected: &[i32]) {
        let k = dedup_sorted(input);
        assert_eq!(k, expected.len(), "incorrect logical length");
        assert_eq!(&input[..k], expected, "incorrect distinct prefix");
        // The tail is deliberately not checked: its contents are unspecified.
    }

    #[test]
    fn empty_input_returns_zero() {
        assert_compacted(&mut [], &[]);
    }

    #[test]
    fn singleton_is_preserved() {
        assert_compacted(&mut [7], &[7]);
        assert_compacted(&mut [i32::MIN], &[i32::MIN]);
    }

    #[test]
    fn equal_pair_becomes_one_value() {
        assert_compacted(&mut [4, 4], &[4]);
    }

    #[test]
    fn distinct_pair_is_preserved() {
        assert_compacted(&mut [-2, 7], &[-2, 7]);
    }

    #[test]
    fn all_equal_values_become_one() {
        assert_compacted(&mut [5, 5, 5, 5, 5], &[5]);
    }

    #[test]
    fn all_distinct_values_are_preserved() {
        assert_compacted(&mut [-4, -1, 0, 3, 8], &[-4, -1, 0, 3, 8]);
    }

    #[test]
    fn duplicate_run_at_start() {
        assert_compacted(&mut [1, 1, 1, 2, 3], &[1, 2, 3]);
    }

    #[test]
    fn duplicate_run_at_end() {
        assert_compacted(&mut [1, 2, 3, 3, 3], &[1, 2, 3]);
    }

    #[test]
    fn unequal_run_lengths_are_compacted() {
        assert_compacted(&mut [1, 1, 2, 3, 3, 3, 4, 4], &[1, 2, 3, 4]);
    }

    #[test]
    fn handles_negative_values_and_zero() {
        assert_compacted(&mut [-5, -5, -2, 0, 0, 7], &[-5, -2, 0, 7]);
    }

    #[test]
    fn handles_both_integer_limits() {
        assert_compacted(
            &mut [i32::MIN, i32::MIN, 0, i32::MAX, i32::MAX],
            &[i32::MIN, 0, i32::MAX],
        );
    }

    #[test]
    fn only_modifies_supplied_subslice() {
        let mut nums = [99, 1, 1, 2, 3, 3, 88];
        assert_compacted(&mut nums[1..6], &[1, 2, 3]);
        assert_eq!(nums[0], 99);
        assert_eq!(nums[6], 88);
    }

    #[test]
    fn reapplying_to_the_logical_prefix_preserves_it() {
        let mut nums = [1, 1, 2, 2, 3];
        let k = dedup_sorted(&mut nums);
        assert_eq!(k, 3);
        assert_eq!(&nums[..k], &[1, 2, 3]);
        assert_compacted(&mut nums[..k], &[1, 2, 3]);
    }
}
