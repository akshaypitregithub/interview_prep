//! G016: Maximum sum of a contiguous fixed-size window.

/// Return the greatest sum of exactly width adjacent elements.
/// Return None if width is zero or greater than nums.len(). Preserve input.
/// Negative sums are valid; do not permit an empty or shorter window.
/// Use i128 arithmetic for sums, widening elements before arithmetic.
/// Use safe Rust and explicit loops; no sorting, prefix-sum vector, or rescanning
/// each complete window. Aim for O(n) time and O(1) auxiliary space.
pub fn max_window_sum(nums: &[i32], width: usize) -> Option<i128> {
    if width > nums.len() || width == 0 {
        return None;
    }

    let mut best_sum_yet: i128 = i128::MIN;
    let mut reusable_sum: Option<i128> = None;
    for i in 0..nums.len() - width + 1 {
        let mut loop_sum: i128 = 0;
        if reusable_sum.is_none() {
            for j in 0..width {
                loop_sum += nums[i + j] as i128
            }
            reusable_sum = Some(loop_sum - nums[i] as i128);
        } else {
            loop_sum = reusable_sum.unwrap() + nums[i + width - 1] as i128;
            reusable_sum = Some(loop_sum - nums[i] as i128);
        }
        if loop_sum > best_sum_yet {
            best_sum_yet = loop_sum;
        }
    }
    Some(best_sum_yet)
}

#[cfg(test)]
mod tests {
    use super::max_window_sum;
    #[test]
    fn empty_input() {
        assert_eq!(max_window_sum(&[], 0), None);
        assert_eq!(max_window_sum(&[], 1), None);
    }
    #[test]
    fn zero_width() {
        assert_eq!(max_window_sum(&[1, 2], 0), None);
    }
    #[test]
    fn oversized_width() {
        assert_eq!(max_window_sum(&[1, 2], 3), None);
        assert_eq!(max_window_sum(&[1], usize::MAX), None);
    }
    #[test]
    fn singleton() {
        assert_eq!(max_window_sum(&[-7], 1), Some(-7));
    }
    #[test]
    fn width_one() {
        assert_eq!(max_window_sum(&[3, -1, 8, 2], 1), Some(8));
    }
    #[test]
    fn whole_slice() {
        assert_eq!(max_window_sum(&[3, -2, 4], 3), Some(5));
    }
    #[test]
    fn maximum_at_start() {
        assert_eq!(max_window_sum(&[9, 8, 1, 0], 2), Some(17));
    }
    #[test]
    fn maximum_in_middle() {
        assert_eq!(max_window_sum(&[1, 8, 9, 2], 2), Some(17));
    }
    #[test]
    fn maximum_at_end() {
        assert_eq!(max_window_sum(&[0, 1, 8, 9], 2), Some(17));
    }
    #[test]
    fn all_negative() {
        assert_eq!(max_window_sum(&[-5, -2, -3, -9], 2), Some(-5));
    }
    #[test]
    fn zeros_and_ties() {
        assert_eq!(max_window_sum(&[0, 0, 0], 2), Some(0));
        assert_eq!(max_window_sum(&[2, 1, 2, 1], 2), Some(3));
    }
    #[test]
    fn adjacency_is_required() {
        assert_eq!(max_window_sum(&[10, -100, 10], 2), Some(-90));
    }
    #[test]
    fn exact_width_is_required() {
        assert_eq!(max_window_sum(&[5, -1, 5], 3), Some(9));
    }
    #[test]
    fn sums_can_exceed_i32() {
        assert_eq!(
            max_window_sum(&[i32::MAX, i32::MAX], 2),
            Some(2 * i128::from(i32::MAX))
        );
        assert_eq!(
            max_window_sum(&[i32::MIN, i32::MIN], 2),
            Some(2 * i128::from(i32::MIN))
        );
    }
    #[test]
    fn transition_between_extremes() {
        assert_eq!(
            max_window_sum(&[i32::MIN, i32::MAX], 1),
            Some(i128::from(i32::MAX))
        );
        assert_eq!(
            max_window_sum(&[i32::MIN, i32::MAX, i32::MAX], 2),
            Some(2 * i128::from(i32::MAX))
        );
    }
    #[test]
    fn supplied_slice_only() {
        let nums = [100, 1, 2, 3, 100];
        assert_eq!(max_window_sum(&nums[1..4], 2), Some(5));
        assert_eq!(nums, [100, 1, 2, 3, 100]);
    }
}
