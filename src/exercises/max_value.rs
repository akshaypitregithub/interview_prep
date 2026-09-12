//! G003: Find the maximum value of a possibly empty borrowed slice.

/// Return `Some(largest_value)`, or `None` for an empty slice.
/// All `i32` values are valid; return a value, not an index or reference.
/// Use an explicit loop, without sorting or built-in maximum/reduction helpers.
/// Do not mutate or copy the slice. Aim for O(n) time and O(1) extra space.
pub fn max_value(nums: &[i32]) -> Option<i32> {
    let mut value: Option<i32> = None;
    for num in nums {
        if let Some(cont) = value {
            if cont < *num {
                value = Some(*num);
            }
        } else {
            value = Some(*num)
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::max_value;

    #[test]
    fn empty_input_returns_none() {
        assert_eq!(max_value(&[]), None);
    }

    #[test]
    fn singleton_returns_its_value() {
        assert_eq!(max_value(&[7]), Some(7));
        assert_eq!(max_value(&[-7]), Some(-7));
        assert_eq!(max_value(&[0]), Some(0));
    }

    #[test]
    fn maximum_at_start() {
        assert_eq!(max_value(&[9, 4, 6]), Some(9));
    }

    #[test]
    fn maximum_in_middle() {
        assert_eq!(max_value(&[4, 9, 6]), Some(9));
    }

    #[test]
    fn maximum_at_end() {
        assert_eq!(max_value(&[4, 6, 9]), Some(9));
    }

    #[test]
    fn all_negative_values() {
        assert_eq!(max_value(&[-8, -3, -5]), Some(-3));
    }

    #[test]
    fn zero_can_be_the_maximum() {
        assert_eq!(max_value(&[-8, 0, -3]), Some(0));
    }

    #[test]
    fn mixed_sign_values() {
        assert_eq!(max_value(&[-9, 4, 0, -2, 7, 3]), Some(7));
    }

    #[test]
    fn repeated_maximum() {
        assert_eq!(max_value(&[8, 3, 8, 4, 8]), Some(8));
    }

    #[test]
    fn all_equal_values() {
        assert_eq!(max_value(&[5, 5, 5]), Some(5));
        assert_eq!(max_value(&[-5, -5, -5]), Some(-5));
    }

    #[test]
    fn integer_minimum_is_a_valid_answer() {
        assert_eq!(max_value(&[i32::MIN]), Some(i32::MIN));
        assert_eq!(max_value(&[i32::MIN, i32::MIN]), Some(i32::MIN));
    }

    #[test]
    fn handles_both_integer_limits() {
        assert_eq!(max_value(&[i32::MIN, 0, i32::MAX]), Some(i32::MAX));
        assert_eq!(max_value(&[i32::MAX, i32::MIN]), Some(i32::MAX));
    }

    #[test]
    fn only_considers_the_supplied_subslice() {
        let nums = [99, -8, -3, -5, 100];
        assert_eq!(max_value(&nums[1..4]), Some(-3));
        assert_eq!(max_value(&nums[2..2]), None);
    }
}
