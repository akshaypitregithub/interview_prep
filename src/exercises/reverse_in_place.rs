//! G004: Reverse a mutable slice in place.

/// Reverse the order of the supplied elements, modifying the caller's data.
/// Empty and single-element slices are valid. Return unit (`()`).
/// Use an explicit loop; `swap` is allowed, but built-in reversal helpers,
/// sorting, recursion, and allocating a second collection are not.
/// Aim for O(n) time and O(1) auxiliary space. Use safe Rust.
pub fn reverse_in_place(nums: &mut [i32]) {
    let size = nums.len();
    if size == 0 || size == 1 {
        return;
    }
    let index_limit = nums.len() - 1;
    for i in 0..size / 2 {
        let temp = nums[i];
        nums[i] = nums[index_limit - i];
        nums[index_limit - i] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_in_place;

    #[test]
    fn empty_input_is_unchanged() {
        let mut nums: [i32; 0] = [];
        reverse_in_place(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn singleton_is_unchanged() {
        let mut nums = [7];
        reverse_in_place(&mut nums);
        assert_eq!(nums, [7]);
    }

    #[test]
    fn swaps_two_elements() {
        let mut nums = [2, 9];
        reverse_in_place(&mut nums);
        assert_eq!(nums, [9, 2]);
    }

    #[test]
    fn reverses_odd_length_and_preserves_middle() {
        let mut nums = [4, 1, 9, 2, 7];
        reverse_in_place(&mut nums);
        assert_eq!(nums, [7, 2, 9, 1, 4]);
    }

    #[test]
    fn reverses_even_length() {
        let mut nums = [4, 1, 9, 2, 7, 3];
        reverse_in_place(&mut nums);
        assert_eq!(nums, [3, 7, 2, 9, 1, 4]);
    }

    #[test]
    fn preserves_duplicate_elements() {
        let mut nums = [2, 2, 7, 2, 9];
        reverse_in_place(&mut nums);
        assert_eq!(nums, [9, 2, 7, 2, 2]);
    }

    #[test]
    fn all_equal_elements_are_unchanged() {
        let mut nums = [5, 5, 5, 5];
        reverse_in_place(&mut nums);
        assert_eq!(nums, [5, 5, 5, 5]);
    }

    #[test]
    fn handles_negatives_zero_and_integer_limits() {
        let mut nums = [i32::MIN, -3, 0, i32::MAX, -8];
        reverse_in_place(&mut nums);
        assert_eq!(nums, [-8, i32::MAX, 0, -3, i32::MIN]);
    }

    #[test]
    fn palindrome_is_unchanged() {
        let mut nums = [2, 8, 3, 8, 2];
        reverse_in_place(&mut nums);
        assert_eq!(nums, [2, 8, 3, 8, 2]);
    }

    #[test]
    fn reversing_twice_restores_original() {
        let mut nums = [4, -2, 0, 9, 4, 7];
        let original = nums;
        reverse_in_place(&mut nums);
        assert_eq!(nums, [7, 4, 9, 0, -2, 4]);
        reverse_in_place(&mut nums);
        assert_eq!(nums, original);
    }

    #[test]
    fn only_modifies_the_supplied_subslice() {
        let mut nums = [99, 4, 1, 9, 2, 100];
        reverse_in_place(&mut nums[1..5]);
        assert_eq!(nums, [99, 2, 9, 1, 4, 100]);
    }

    #[test]
    fn empty_subslice_leaves_surroundings_unchanged() {
        let mut nums = [4, 1, 9];
        reverse_in_place(&mut nums[1..1]);
        assert_eq!(nums, [4, 1, 9]);
    }

    #[test]
    fn accepts_a_borrowed_vector_without_changing_length() {
        let mut nums = vec![6, 3, 8, 1];
        reverse_in_place(&mut nums);
        assert_eq!(nums, vec![1, 8, 3, 6]);
    }
}
