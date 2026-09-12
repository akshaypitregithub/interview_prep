//! G008: Merge two sorted borrowed slices, retaining all duplicates.

/// Both inputs are sorted in nondecreasing order; unsorted input is out of scope.
/// Return a sorted vector containing every input element, including duplicates.
/// Do not mutate inputs. Empty inputs and all i32 values are valid.
/// Use explicit loops, not concatenation followed by sorting or merge helpers.
/// Aim for O(n + m) time and O(1) auxiliary space excluding the output vector.
pub fn merge_sorted(left: &[i32], right: &[i32]) -> Vec<i32> {
    if left.is_empty() {
        return Vec::from(right);
    } else if right.is_empty() {
        return Vec::from(left);
    }
    let left_len = left.len();
    let right_len = right.len();
    let total = left_len + right_len;
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut output = Vec::<i32>::with_capacity(total);
    for _ in 0..total {
        if left_idx < left_len && (right_idx == right_len || left[left_idx] < right[right_idx]) {
            output.push(left[left_idx]);
            left_idx += 1
        } else if right_idx < right_len
            && (left_idx == left_len || left[left_idx] > right[right_idx])
        {
            output.push(right[right_idx]);
            right_idx += 1
        } else if right_idx < right_len && left_idx < left_len {
            output.push(right[right_idx]);
            output.push(left[left_idx]);
            left_idx += 1;
            right_idx += 1
        }
        if left_idx == left_len && right_idx == right_len {
            break;
        }
    }
    output
}

// [10, 11, 15, 16]
// [2, 4, 5, 6]
//

#[cfg(test)]
mod tests {
    use super::merge_sorted;

    #[test]
    fn both_empty_returns_empty() {
        assert_eq!(merge_sorted(&[], &[]), vec![]);
    }

    #[test]
    fn empty_left_preserves_right() {
        assert_eq!(merge_sorted(&[], &[-2, 0, 3, 3]), vec![-2, 0, 3, 3]);
    }

    #[test]
    fn empty_right_preserves_left() {
        assert_eq!(merge_sorted(&[-2, 0, 3, 3], &[]), vec![-2, 0, 3, 3]);
    }

    #[test]
    fn singleton_inputs_work_in_either_order() {
        assert_eq!(merge_sorted(&[2], &[7]), vec![2, 7]);
        assert_eq!(merge_sorted(&[7], &[2]), vec![2, 7]);
    }

    #[test]
    fn equal_singletons_are_both_retained() {
        assert_eq!(merge_sorted(&[4], &[4]), vec![4, 4]);
    }

    #[test]
    fn interleaves_inputs() {
        assert_eq!(merge_sorted(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn preserves_right_remainder() {
        assert_eq!(merge_sorted(&[1, 2], &[3, 4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn preserves_left_remainder() {
        assert_eq!(merge_sorted(&[3, 4, 5, 6], &[1, 2]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn handles_unequal_lengths_with_interleaving() {
        assert_eq!(merge_sorted(&[2], &[1, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
        assert_eq!(merge_sorted(&[1, 3, 4, 5], &[2]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn retains_duplicates_within_and_across_inputs() {
        assert_eq!(
            merge_sorted(&[1, 1, 3], &[1, 2, 3, 3]),
            vec![1, 1, 1, 2, 3, 3, 3]
        );
        assert_eq!(merge_sorted(&[5, 5], &[5, 5, 5]), vec![5, 5, 5, 5, 5]);
    }

    #[test]
    fn handles_negatives_zero_and_integer_limits() {
        assert_eq!(
            merge_sorted(&[i32::MIN, -3, 0, i32::MAX], &[i32::MIN, -2, 0, i32::MAX]),
            vec![i32::MIN, i32::MIN, -3, -2, 0, 0, i32::MAX, i32::MAX]
        );
    }

    #[test]
    fn only_uses_supplied_subslices() {
        let left = [99, 1, 4, 88];
        let right = [77, 2, 3, 66];
        assert_eq!(merge_sorted(&left[1..3], &right[1..3]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn allows_two_views_of_the_same_input() {
        let nums = [1, 2, 3];
        assert_eq!(merge_sorted(&nums, &nums), vec![1, 1, 2, 2, 3, 3]);
    }
}
