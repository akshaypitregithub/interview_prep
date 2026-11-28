//! G024: Minimum meeting rooms.

/// Return the minimum number of meeting rooms needed for all meetings.
/// Each tuple is a half-open interval [start, end), with start < end guaranteed.
/// Input may be unsorted, duplicated, nested, or include negative/extreme timestamps.
/// Touching endpoints do NOT overlap: (1, 3) and (3, 5) share 1 room.
/// Overlapping intervals require separate rooms: (1, 4) and (3, 5) need 2 rooms.
/// Empty input returns 0. Preserve the borrowed input.
/// Support the full i32 range. Use safe Rust, O(n log(n + 1)) time,
/// and O(n) auxiliary space. Standard sorting, copying input, and standard
/// collections are allowed. Use an explicit loop for the sweep/counting pass;
/// no coordinate enumeration, recursion, or calls to earlier exercises.
pub fn min_meeting_rooms(meetings: &[(i32, i32)]) -> usize {
    let mut active: i32 = 0;
    let mut peak: i32 = 0;
    let mut nested_interval_tuple = Vec::new();
    meetings.iter().for_each(|meeting| {
        nested_interval_tuple.push((meeting.0, 1));
        nested_interval_tuple.push((meeting.1, -1))
    });
    nested_interval_tuple.sort_by(|x, y| x.0.cmp(&y.0).then(x.1.cmp(&y.1)));
    for meeting in nested_interval_tuple {
        active += meeting.1;
        if active > peak {
            peak = active;
        }
    }
    peak as usize
}

#[cfg(test)]
mod tests {
    use super::min_meeting_rooms;

    #[test]
    fn empty_input() {
        assert_eq!(min_meeting_rooms(&[]), 0);
    }

    #[test]
    fn single_meeting() {
        assert_eq!(min_meeting_rooms(&[(2, 7)]), 1);
    }

    #[test]
    fn unsorted_overlapping_meetings() {
        assert_eq!(min_meeting_rooms(&[(0, 30), (15, 20), (5, 10)]), 2);
    }

    #[test]
    fn disjoint_meetings_need_one_room() {
        assert_eq!(min_meeting_rooms(&[(8, 9), (-5, -2), (2, 4)]), 1);
    }

    #[test]
    fn touching_endpoints_share_a_room() {
        assert_eq!(min_meeting_rooms(&[(3, 5), (1, 3)]), 1);
        assert_eq!(min_meeting_rooms(&[(1, 2), (2, 3), (3, 4)]), 1);
    }

    #[test]
    fn multiple_endings_and_starts_at_same_timestamp() {
        assert_eq!(min_meeting_rooms(&[(5, 9), (1, 5), (5, 8), (2, 5)]), 2);
    }

    #[test]
    fn peak_occurs_before_end_of_schedule() {
        assert_eq!(
            min_meeting_rooms(&[(6, 8), (1, 5), (3, 4), (7, 9), (2, 6)]),
            3
        );
    }

    #[test]
    fn nested_meetings_stack_rooms() {
        assert_eq!(min_meeting_rooms(&[(1, 10), (2, 8), (3, 5), (4, 5)]), 4);
    }

    #[test]
    fn duplicate_identical_meetings() {
        assert_eq!(min_meeting_rooms(&[(2, 6), (2, 6), (2, 6)]), 3);
    }

    #[test]
    fn equal_starts_and_equal_ends() {
        assert_eq!(min_meeting_rooms(&[(2, 4), (2, 9), (2, 6)]), 3);
        assert_eq!(min_meeting_rooms(&[(1, 8), (4, 8), (6, 8)]), 3);
    }

    #[test]
    fn negative_and_cross_zero_timestamps() {
        assert_eq!(min_meeting_rooms(&[(-2, 4), (-10, -2), (3, 7), (-5, 3)]), 2);
    }

    #[test]
    fn extreme_i32_endpoints() {
        assert_eq!(
            min_meeting_rooms(&[(i32::MIN, i32::MIN + 1), (i32::MAX - 1, i32::MAX)]),
            1
        );
        assert_eq!(
            min_meeting_rooms(&[(0, i32::MAX), (i32::MIN, 0), (i32::MIN, i32::MAX)]),
            2
        );
    }

    #[test]
    fn borrowed_subslice_is_preserved() {
        let meetings = [(0, 100), (1, 5), (4, 7), (5, 9), (-10, -1)];
        let original = meetings;
        assert_eq!(min_meeting_rooms(&meetings[1..4]), 2);
        assert_eq!(meetings, original);
    }

    #[test]
    fn larger_reverse_order_inputs() {
        let chain: Vec<_> = (0..10_000).rev().map(|i| (i, i + 1)).collect();
        assert_eq!(min_meeting_rooms(&chain), 1);

        let nested: Vec<_> = (0..5_000).rev().map(|i| (i, 10_000 - i)).collect();
        assert_eq!(min_meeting_rooms(&nested), 5_000);

        let staggered: Vec<_> = (0..10_000).rev().map(|i| (i, i + 3)).collect();
        assert_eq!(min_meeting_rooms(&staggered), 3);
    }

    #[test]
    fn small_schedules_match_pointwise_definition() {
        let all_intervals: Vec<(i32, i32)> = (0..5)
            .flat_map(|s| ((s + 1)..=5).map(move |e| (s, e)))
            .collect();

        let pointwise_max = |schedule: &[(i32, i32)]| -> usize {
            (0..5)
                .map(|t| schedule.iter().filter(|&&(s, e)| s <= t && t < e).count())
                .max()
                .unwrap_or(0)
        };

        for &a in &all_intervals {
            for &b in &all_intervals {
                for &c in &all_intervals {
                    let schedule = [c, a, b];
                    assert_eq!(
                        min_meeting_rooms(&schedule),
                        pointwise_max(&schedule),
                        "failed on {schedule:?}"
                    );
                }
            }
        }
    }
}
