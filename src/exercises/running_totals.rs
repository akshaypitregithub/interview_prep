//! G006: Build running totals and report the first overflowing prefix.

/// Return one inclusive prefix sum per input element, in the same order.
/// Empty input returns Ok(empty vector). Do not mutate the input.
/// If a prefix sum cannot fit in i32, return Err(first_overflowing_index).
/// Even if later values would bring the total back into range, return that error.
/// Use an explicit loop; checked arithmetic is allowed. Never panic or wrap.
/// Aim for O(n) time and O(1) auxiliary space excluding the O(n) output.
pub fn running_totals(nums: &[i32]) -> Result<Vec<i32>, usize> {
    let mut output = Vec::<i32>::with_capacity(nums.len());
    let mut running_total: i32 = 0;
    for (i, num) in nums.iter().enumerate() {
        if let Some(val) = running_total.checked_add(*num) {
            running_total = val;
        } else {
            return Err(i);
        }
        output.push(running_total);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::running_totals;

    #[test]
    fn empty_input_returns_empty_vector() {
        assert_eq!(running_totals(&[]), Ok(vec![]));
    }

    #[test]
    fn singleton_preserves_value_including_limits() {
        for value in [0, 7, -7, i32::MIN, i32::MAX] {
            assert_eq!(running_totals(&[value]), Ok(vec![value]));
        }
    }

    #[test]
    fn produces_inclusive_totals() {
        assert_eq!(running_totals(&[2, 3, 4]), Ok(vec![2, 5, 9]));
    }

    #[test]
    fn handles_negative_values() {
        assert_eq!(running_totals(&[-2, -3, -4]), Ok(vec![-2, -5, -9]));
    }

    #[test]
    fn handles_zero_and_repeated_values() {
        assert_eq!(running_totals(&[0, 2, 2, 0]), Ok(vec![0, 2, 4, 4]));
    }

    #[test]
    fn handles_cancellation() {
        assert_eq!(running_totals(&[4, -4, -2, 5]), Ok(vec![4, 0, -2, 3]));
    }

    #[test]
    fn accepts_exact_integer_boundaries() {
        assert_eq!(
            running_totals(&[i32::MAX - 1, 1, 0]),
            Ok(vec![i32::MAX - 1, i32::MAX, i32::MAX])
        );
        assert_eq!(
            running_totals(&[i32::MIN + 1, -1, 0]),
            Ok(vec![i32::MIN + 1, i32::MIN, i32::MIN])
        );
    }

    #[test]
    fn rejects_positive_overflow() {
        assert_eq!(running_totals(&[i32::MAX, 1]), Err(1));
    }

    #[test]
    fn rejects_negative_overflow() {
        assert_eq!(running_totals(&[i32::MIN, -1]), Err(1));
    }

    #[test]
    fn reports_the_first_overflowing_index() {
        assert_eq!(running_totals(&[0, i32::MAX, 0, 1, 1]), Err(3));
    }

    #[test]
    fn later_cancellation_does_not_hide_overflow() {
        assert_eq!(running_totals(&[i32::MAX, 1, -1]), Err(1));
        assert_eq!(running_totals(&[i32::MIN, -1, 1]), Err(1));
    }

    #[test]
    fn opposite_integer_limits_can_be_added_safely() {
        assert_eq!(
            running_totals(&[i32::MAX, i32::MIN, 1]),
            Ok(vec![i32::MAX, -1, 0])
        );
    }

    #[test]
    fn totals_and_error_indices_are_relative_to_subslice() {
        let nums = [99, 2, 3, 4, 100];
        assert_eq!(running_totals(&nums[1..4]), Ok(vec![2, 5, 9]));
        let overflowing = [99, i32::MAX, 1, -1];
        assert_eq!(running_totals(&overflowing[1..]), Err(1));
    }
}
