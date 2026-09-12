//! G015: Length of the longest consecutive run of distinct integer values.

use std::collections::HashSet;

/// Return the longest run length among values present, regardless of input order.
/// Duplicates do not extend a run. Empty input returns zero. Preserve input.
/// Consecutive means mathematical difference one; MIN and MAX do not wrap together.
/// Use safe Rust, explicit loops and standard library collections; no sorting or
/// earlier exercise calls. Aim for expected O(n) time and O(k) auxiliary space,
/// where k is the number of distinct values. Repeatedly rescanning runs is disallowed.
pub fn longest_consecutive(nums: &[i32]) -> usize {
    let mut set: HashSet<i32> = HashSet::new();

    let mut longest_run_yet: usize = 1;

    for num in nums {
        set.insert(*num);
    }

    if set.is_empty() {
        return 0;
    }

    for item in set.iter() {
        let previous_run_check = item.checked_sub(1);
        if let Some(previous_run_item) = previous_run_check {
            let is_start = !set.contains(&(previous_run_item));

            if is_start {
                let mut run_length = 1;

                loop {
                    let i = item.checked_add(run_length);
                    if let Some(effective_length) = i {
                        if !set.contains(&(effective_length)) {
                            break;
                        } else {
                            run_length += 1;
                            continue;
                        }
                    } else {
                        break;
                    }
                }

                let run_length_usize = run_length as usize;

                if run_length_usize > longest_run_yet {
                    longest_run_yet = run_length_usize
                }
            }
        } else {
            let mut run_length = 1;

            loop {
                let i = item.checked_add(run_length);
                if let Some(effective_length) = i {
                    if !set.contains(&(effective_length)) {
                        break;
                    } else {
                        run_length += 1;
                        continue;
                    }
                } else {
                    break;
                }
            }

            let run_length_usize = run_length as usize;

            if run_length_usize > longest_run_yet {
                longest_run_yet = run_length_usize
            }
        }
    }

    longest_run_yet
}

#[cfg(test)]
mod tests {
    use super::longest_consecutive;
    #[test]
    fn empty_input() {
        assert_eq!(longest_consecutive(&[]), 0);
    }
    #[test]
    fn singleton() {
        assert_eq!(longest_consecutive(&[42]), 1);
    }
    #[test]
    fn duplicates_alone() {
        assert_eq!(longest_consecutive(&[7, 7, 7]), 1);
    }
    #[test]
    fn unordered_run() {
        assert_eq!(longest_consecutive(&[100, 4, 200, 1, 3, 2]), 4);
    }
    #[test]
    fn increasing_run() {
        assert_eq!(longest_consecutive(&[1, 2, 3, 4, 5]), 5);
    }
    #[test]
    fn decreasing_run() {
        assert_eq!(longest_consecutive(&[5, 4, 3, 2, 1]), 5);
    }
    #[test]
    fn gaps_break_runs() {
        assert_eq!(longest_consecutive(&[1, 3, 5, 7]), 1);
    }
    #[test]
    fn longest_is_not_first_run() {
        assert_eq!(longest_consecutive(&[1, 2, 10, 11, 12, 13]), 4);
    }
    #[test]
    fn tied_runs() {
        assert_eq!(longest_consecutive(&[8, 9, 1, 2]), 2);
    }
    #[test]
    fn duplicates_do_not_extend_run() {
        assert_eq!(longest_consecutive(&[1, 2, 2, 3, 1, 4, 4]), 4);
    }
    #[test]
    fn negative_values() {
        assert_eq!(longest_consecutive(&[-4, -2, -3, -10]), 3);
    }
    #[test]
    fn crosses_zero() {
        assert_eq!(longest_consecutive(&[2, -1, 0, -2, 1]), 5);
    }
    #[test]
    fn minimum_boundary() {
        assert_eq!(longest_consecutive(&[i32::MIN + 1, i32::MIN]), 2);
    }
    #[test]
    fn maximum_boundary() {
        assert_eq!(longest_consecutive(&[i32::MAX, i32::MAX - 1]), 2);
    }
    #[test]
    fn extremes_are_not_neighbors() {
        assert_eq!(longest_consecutive(&[i32::MIN, i32::MAX]), 1);
        assert_eq!(
            longest_consecutive(&[i32::MAX - 1, i32::MAX, i32::MIN, i32::MIN + 1]),
            2
        );
    }
    #[test]
    fn supplied_slice_only() {
        let nums = [0, 1, 2, 3, 4];
        assert_eq!(longest_consecutive(&nums[1..4]), 3);
        assert_eq!(nums, [0, 1, 2, 3, 4]);
    }
    #[test]
    fn duplicate_run_starts() {
        let mut nums = vec![0; 1000];
        nums.extend(1..1000);
        assert_eq!(longest_consecutive(&nums), 1000);
    }
}
