pub fn min_rooms(meetings: Vec<Vec<i64>>) -> i64 {
    let mut starts: Vec<i64> = meetings.iter().map(|m| m[0]).collect();
    let mut ends: Vec<i64> = meetings.iter().map(|m| m[1]).collect();
    starts.sort_unstable();
    ends.sort_unstable();
    let (mut rooms, mut best, mut j) = (0i64, 0i64, 0usize);
    for s in starts {
        while ends[j] <= s {
            rooms -= 1;
            j += 1;
        }
        rooms += 1;
        best = best.max(rooms);
    }
    best
}
