//! G023: Merge closed intervals.

/// Return the union as sorted, nonoverlapping closed intervals.
/// Each tuple is (start, end), with start <= end guaranteed.
/// Input may be unsorted, duplicated, nested, negative, or zero-length.
/// Shared endpoints merge: (1, 3) and (3, 5) become (1, 5).
/// Gaps do not merge: (1, 2) and (3, 4) remain separate.
/// Output is sorted by start, with previous.end < next.start.
/// Empty input returns an empty Vec. Preserve the borrowed input.
/// Support the full i32 range. Use safe Rust, O(n log(n + 1)) time,
/// and O(n) auxiliary space. Standard sorting and copying input are allowed.
/// Use an explicit merging loop; no coordinate enumeration, recursion,
/// or calls to earlier exercises.
pub fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let _ = intervals;
    todo!("Implement merging closed intervals")
}

#[cfg(test)]
mod tests {
    use super::merge_intervals;

    #[test]
    fn empty_input() {
        assert_eq!(merge_intervals(&[]), vec![]);
    }

    #[test]
    fn single_interval() {
        assert_eq!(merge_intervals(&[(2, 7)]), vec![(2, 7)]);
    }

    #[test]
    fn merges_unsorted_overlaps() {
        assert_eq!(
            merge_intervals(&[(8, 10), (2, 6), (15, 18), (1, 3)]),
            vec![(1, 6), (8, 10), (15, 18)]
        );
    }

    #[test]
    fn sorts_disjoint_intervals() {
        assert_eq!(
            merge_intervals(&[(8, 9), (-5, -2), (2, 4)]),
            vec![(-5, -2), (2, 4), (8, 9)]
        );
    }

    #[test]
    fn touching_endpoints_merge() {
        assert_eq!(merge_intervals(&[(3, 5), (1, 3)]), vec![(1, 5)]);
    }

    #[test]
    fn adjacent_integer_endpoints_do_not_merge() {
        assert_eq!(merge_intervals(&[(1, 2), (3, 4)]), vec![(1, 2), (3, 4)]);
    }

    #[test]
    fn nested_intervals_do_not_shrink_outer_end() {
        assert_eq!(
            merge_intervals(&[(1, 10), (2, 3), (4, 8), (9, 12)]),
            vec![(1, 12)]
        );
    }

    #[test]
    fn equal_starts_and_duplicates() {
        assert_eq!(
            merge_intervals(&[(2, 4), (2, 9), (2, 3), (2, 9)]),
            vec![(2, 9)]
        );
    }

    #[test]
    fn equal_ends() {
        assert_eq!(merge_intervals(&[(4, 8), (1, 8), (3, 8)]), vec![(1, 8)]);
    }

    #[test]
    fn transitive_chain_merges() {
        assert_eq!(
            merge_intervals(&[(6, 9), (1, 4), (3, 7), (9, 11)]),
            vec![(1, 11)]
        );
    }

    #[test]
    fn zero_length_intervals() {
        assert_eq!(merge_intervals(&[(0, 0)]), vec![(0, 0)]);
        assert_eq!(
            merge_intervals(&[(3, 3), (1, 3), (2, 2), (5, 5), (5, 5)]),
            vec![(1, 3), (5, 5)]
        );
    }

    #[test]
    fn negative_and_cross_zero_intervals() {
        assert_eq!(
            merge_intervals(&[(-4, 2), (-8, -4), (1, 6), (-12, -10)]),
            vec![(-12, -10), (-8, 6)]
        );
    }

    #[test]
    fn extreme_endpoints() {
        assert_eq!(
            merge_intervals(&[(i32::MAX, i32::MAX), (i32::MIN, i32::MIN)]),
            vec![(i32::MIN, i32::MIN), (i32::MAX, i32::MAX)]
        );
        assert_eq!(
            merge_intervals(&[(0, i32::MAX), (i32::MIN, 0)]),
            vec![(i32::MIN, i32::MAX)]
        );
    }

    #[test]
    fn borrowed_subslice_is_preserved() {
        let intervals = [(99, 100), (4, 7), (1, 5), (-10, -9)];
        let original = intervals;
        assert_eq!(merge_intervals(&intervals[1..3]), vec![(1, 7)]);
        assert_eq!(intervals, original);
    }

    #[test]
    fn larger_reverse_order_input() {
        let intervals: Vec<_> = (0..10_000).rev().map(|i| (i, i + 1)).collect();
        assert_eq!(merge_intervals(&intervals), vec![(0, 10_000)]);
        let disjoint: Vec<_> = (0..10_000).rev().map(|i| (3 * i, 3 * i + 1)).collect();
        let expected: Vec<_> = (0..10_000).map(|i| (3 * i, 3 * i + 1)).collect();
        assert_eq!(merge_intervals(&disjoint), expected);
    }
}
