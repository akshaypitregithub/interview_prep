//! G022: Minimum capacity to ship packages in order.

/// Return the smallest daily capacity that ships all weights within `days` days.
/// Packages cannot be split or reordered. Each used day ships a contiguous
/// group of remaining packages whose total weight is at most the capacity.
/// Unused days are allowed; zero-weight packages are valid.
/// Empty input returns Some(0), even with zero days. Nonempty input with zero
/// days returns None (including all-zero weights). Otherwise return Some(capacity).
/// The mathematical sum of weights is guaranteed to fit u64. Preserve input.
/// Use safe Rust, explicit loops, O(n * log(S + 2)) time and O(1) auxiliary space,
/// where n is the number of packages and S is their total weight.
/// No sorting, recursion, floating point, scanning every possible capacity,
/// built-in search/partition helpers, or calls to earlier exercises.
pub fn shipping_capacity(weights: &[u32], days: usize) -> Option<u64> {
    let _ = (weights, days);
    todo!("Implement minimum shipping capacity")
}

#[cfg(test)]
mod tests {
    use super::shipping_capacity;

    // Independent small-input oracle: enumerate every placement of cuts
    // between packages, then compare the heaviest group in each partition.
    fn partition_oracle(weights: &[u32], days: usize) -> Option<u64> {
        if weights.is_empty() {
            return Some(0);
        }
        if days == 0 {
            return None;
        }
        assert!(weights.len() <= 6);
        let mut best = u64::MAX;
        for cuts in 0_usize..(1 << (weights.len() - 1)) {
            if cuts.count_ones() as usize + 1 > days {
                continue;
            }
            let mut load = 0;
            let mut heaviest = 0;
            for (index, &weight) in weights.iter().enumerate() {
                load += u64::from(weight);
                if index + 1 == weights.len() || cuts & (1 << index) != 0 {
                    heaviest = heaviest.max(load);
                    load = 0;
                }
            }
            best = best.min(heaviest);
        }
        Some(best)
    }

    #[test]
    fn empty_input_needs_no_capacity() {
        for days in [0, 1, 10, usize::MAX] {
            assert_eq!(shipping_capacity(&[], days), Some(0));
        }
    }

    #[test]
    fn nonempty_input_cannot_ship_in_zero_days() {
        for weights in [&[7][..], &[0][..], &[0, 0][..], &[1, 2][..]] {
            assert_eq!(shipping_capacity(weights, 0), None);
        }
    }

    #[test]
    fn single_package() {
        for days in [1, 2, usize::MAX] {
            assert_eq!(shipping_capacity(&[7], days), Some(7));
        }
    }

    #[test]
    fn one_day_requires_total_weight() {
        assert_eq!(shipping_capacity(&[3, 1, 4, 2], 1), Some(10));
    }

    #[test]
    fn enough_days_for_each_package() {
        for days in [4, 5, usize::MAX] {
            assert_eq!(shipping_capacity(&[3, 1, 4, 2], days), Some(4));
        }
    }

    #[test]
    fn finds_minimum_for_multiple_days() {
        assert_eq!(
            shipping_capacity(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            Some(15)
        );
        assert_eq!(shipping_capacity(&[3, 2, 2, 4, 1, 4], 3), Some(6));
    }

    #[test]
    fn exact_capacity_loads_fit() {
        assert_eq!(shipping_capacity(&[2, 3, 2, 3], 2), Some(5));
    }

    #[test]
    fn order_changes_the_answer() {
        assert_eq!(shipping_capacity(&[1, 1, 4, 4], 2), Some(6));
        assert_eq!(shipping_capacity(&[1, 4, 1, 4], 2), Some(5));
    }

    #[test]
    fn packages_cannot_be_split() {
        assert_eq!(shipping_capacity(&[1, 10, 1], 2), Some(11));
        assert_eq!(shipping_capacity(&[1, 10, 1], 3), Some(10));
    }

    #[test]
    fn repeated_weights_and_unused_days() {
        assert_eq!(shipping_capacity(&[5, 5, 5, 5, 5], 3), Some(10));
        assert_eq!(shipping_capacity(&[5, 5, 5, 5, 5], 4), Some(10));
    }

    #[test]
    fn zero_weight_packages() {
        assert_eq!(shipping_capacity(&[0], 1), Some(0));
        assert_eq!(shipping_capacity(&[0, 0, 0], 1), Some(0));
        assert_eq!(shipping_capacity(&[0, 3, 0, 2, 0], 2), Some(3));
    }

    #[test]
    fn total_exceeds_u32_range() {
        let max = u32::MAX;
        assert_eq!(
            shipping_capacity(&[max, max, max], 1),
            Some(3 * u64::from(max))
        );
        assert_eq!(
            shipping_capacity(&[max, max, max], 2),
            Some(2 * u64::from(max))
        );
        assert_eq!(shipping_capacity(&[max, max, max], 3), Some(u64::from(max)));
    }

    #[test]
    fn borrowed_subslice_is_preserved() {
        let weights = [99, 1, 2, 3, 99];
        assert_eq!(shipping_capacity(&weights[1..4], 2), Some(3));
        assert_eq!(weights, [99, 1, 2, 3, 99]);
    }

    #[test]
    fn exhaustive_small_inputs_match_partitions() {
        for len in 0..=6_u32 {
            for mut encoding in 0..4_usize.pow(len) {
                let mut weights = vec![0; len as usize];
                for weight in &mut weights {
                    *weight = (encoding % 4) as u32;
                    encoding /= 4;
                }
                for days in 0..=weights.len() + 1 {
                    assert_eq!(
                        shipping_capacity(&weights, days),
                        partition_oracle(&weights, days),
                        "weights={weights:?}, days={days}"
                    );
                }
            }
        }
    }

    #[test]
    fn larger_input() {
        let weights = vec![1; 10_000];
        assert_eq!(shipping_capacity(&weights, 3), Some(3334));
        assert_eq!(shipping_capacity(&weights, 100), Some(100));
    }
}
