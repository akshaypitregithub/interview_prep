//! G014: Distinct intersection in left-input order.

use std::collections::HashMap;

/// Return each value present in both slices exactly once, in order of its first
/// occurrence in left. Preserve inputs. Empty intersection gives an empty Vec.
/// Use safe Rust, explicit loops and standard library collections; no sorting,
/// nested full scans, or calls to earlier exercises. Aim for expected O(n + m)
/// time and O(k) auxiliary space for k distinct values in right, excluding output.
/// Do not reserve output capacity proportional to input lengths when few values match.
pub fn ordered_intersection(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, bool> = HashMap::new();

    for i in 0..left.len() + right.len() {
        if i < right.len() {
            if let Some(_) = map.get(&right[i]) {
                continue;
            } else {
                map.insert(right[i], true);
            }
        } else {
            if let Some(b) = map.get(&left[i - right.len()]) {
                if *b {
                    output.push(left[i - right.len()]);
                    map.insert(left[i - right.len()], false);
                }
            } else {
                continue;
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::ordered_intersection;
    #[test]
    fn both_empty() {
        assert_eq!(ordered_intersection(&[], &[]), vec![]);
    }
    #[test]
    fn one_empty() {
        assert_eq!(ordered_intersection(&[1], &[]), vec![]);
        assert_eq!(ordered_intersection(&[], &[1]), vec![]);
    }
    #[test]
    fn singleton_match() {
        assert_eq!(ordered_intersection(&[7], &[7]), vec![7]);
    }
    #[test]
    fn disjoint_inputs() {
        assert_eq!(ordered_intersection(&[1, 3], &[2, 4]), vec![]);
    }
    #[test]
    fn partial_overlap() {
        assert_eq!(ordered_intersection(&[1, 2, 3, 4], &[2, 4, 6]), vec![2, 4]);
    }
    #[test]
    fn left_order_not_numeric_or_right_order() {
        assert_eq!(ordered_intersection(&[8, 2, 5], &[5, 2, 8]), vec![8, 2, 5]);
    }
    #[test]
    fn repeated_left_values_appear_once() {
        assert_eq!(
            ordered_intersection(&[3, 1, 3, 2, 1], &[1, 2, 3]),
            vec![3, 1, 2]
        );
    }
    #[test]
    fn repeated_right_values_appear_once() {
        assert_eq!(ordered_intersection(&[2, 1], &[1, 1, 2, 2]), vec![2, 1]);
    }
    #[test]
    fn duplicates_on_both_sides() {
        assert_eq!(ordered_intersection(&[4, 4, 4], &[4, 4]), vec![4]);
    }
    #[test]
    fn nonmatches_do_not_affect_order() {
        assert_eq!(
            ordered_intersection(&[9, 3, 9, 1, 3, 7, 2], &[2, 3, 1]),
            vec![3, 1, 2]
        );
    }
    #[test]
    fn negative_values_and_zero() {
        assert_eq!(
            ordered_intersection(&[-2, 0, -1, -2], &[-1, -2, 0]),
            vec![-2, 0, -1]
        );
    }
    #[test]
    fn integer_extremes() {
        assert_eq!(
            ordered_intersection(&[i32::MAX, i32::MIN, i32::MAX], &[i32::MIN, i32::MAX]),
            vec![i32::MAX, i32::MIN]
        );
    }
    #[test]
    fn swapping_inputs_can_change_output_order() {
        assert_eq!(ordered_intersection(&[1, 2], &[2, 1]), vec![1, 2]);
        assert_eq!(ordered_intersection(&[2, 1], &[1, 2]), vec![2, 1]);
    }
    #[test]
    fn only_uses_supplied_slices() {
        let left = [99, 3, 1, 2, 88];
        let right = [99, 2, 3, 88];
        assert_eq!(ordered_intersection(&left[1..4], &right[1..3]), vec![3, 2]);
        assert_eq!(left, [99, 3, 1, 2, 88]);
        assert_eq!(right, [99, 2, 3, 88]);
    }
}
