//! G021: Integer square root with safe arithmetic.

/// Return floor(sqrt(n)): the greatest integer r whose mathematical square <= n.
/// Support every u64 input, including zero and u64::MAX, without overflow/panic.
/// Use safe Rust, explicit loops, O(log(n + 1)) time, and O(1) auxiliary space.
/// No floating point, built-in square-root/search helpers, recursion, linear
/// scan, wider integer types, or calls to earlier exercises in the implementation.
/// Checked integer arithmetic is allowed. Tests may use u128 as an oracle.
pub fn integer_sqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut left = 1;
    let mut right = n;
    let mut res = 1;

    while right >= left {
        let mid = left + (right - left) / 2;

        let checked_square = mid.checked_mul(mid);

        if let Some(square) = checked_square {
            if square <= n {
                left = mid + 1;
                res = mid;
            } else {
                right = mid - 1;
            }
        } else {
            right = mid - 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::integer_sqrt;

    fn assert_floor_root(n: u64) {
        let root = u128::from(integer_sqrt(n));
        let input = u128::from(n);
        // Check this bound first so a broken implementation cannot overflow
        // the oracle's (root + 1) square.
        assert!(root <= u128::from(u32::MAX), "n={n}, root={root}");
        assert!(root * root <= input, "root too large: n={n}, root={root}");
        assert!(
            (root + 1) * (root + 1) > input,
            "root too small: n={n}, root={root}"
        );
    }

    #[test]
    fn zero() {
        assert_eq!(integer_sqrt(0), 0);
    }

    #[test]
    fn one() {
        assert_eq!(integer_sqrt(1), 1);
    }

    #[test]
    fn two_and_three_round_down() {
        for n in [2, 3] {
            assert_eq!(integer_sqrt(n), 1);
        }
    }

    #[test]
    fn small_perfect_squares() {
        for root in 2..=100_u64 {
            assert_eq!(integer_sqrt(root * root), root);
        }
    }

    #[test]
    fn immediately_below_squares() {
        for root in [2, 3, 10, 101, 65_536, 1_000_000_u64] {
            assert_eq!(integer_sqrt(root * root - 1), root - 1);
        }
    }

    #[test]
    fn immediately_above_squares() {
        for root in [2, 3, 10, 101, 65_536, 1_000_000_u64] {
            assert_eq!(integer_sqrt(root * root + 1), root);
        }
    }

    #[test]
    fn rounds_down_between_squares() {
        for (n, expected) in [(8, 2), (15, 3), (24, 4), (80, 8), (12345, 111)] {
            assert_eq!(integer_sqrt(n), expected);
        }
    }

    #[test]
    fn crosses_u32_input_boundary() {
        let boundary = u64::from(u32::MAX);
        assert_eq!(integer_sqrt(boundary), 65_535);
        assert_eq!(integer_sqrt(boundary + 1), 65_536);
        assert_eq!(integer_sqrt(boundary + 2), 65_536);
    }

    #[test]
    fn largest_representable_square_and_neighbors() {
        let root = u64::from(u32::MAX);
        let square = root * root;
        assert_eq!(integer_sqrt(square - 1), root - 1);
        assert_eq!(integer_sqrt(square), root);
        assert_eq!(integer_sqrt(square + 1), root);
    }

    #[test]
    fn maximum_input_and_neighbor() {
        assert_eq!(integer_sqrt(u64::MAX), u64::from(u32::MAX));
        assert_eq!(integer_sqrt(u64::MAX - 1), u64::from(u32::MAX));
    }

    #[test]
    fn powers_of_two_and_neighbors() {
        for shift in 0..64 {
            let n = 1_u64 << shift;
            for input in [n - 1, n, n + 1] {
                assert_floor_root(input);
            }
        }
    }

    #[test]
    fn exhaustive_small_inputs() {
        for n in 0..=10_000 {
            assert_floor_root(n);
        }
    }

    #[test]
    fn deterministic_samples_across_u64_range() {
        let mut n = 42_u64;
        for _ in 0..2048 {
            // Wrapping is intentional only in this test-data generator.
            n = n.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            assert_floor_root(n);
        }
    }
}
