pub fn min_rooms(meetings: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n²): for every meeting, count the meetings running when it starts.
    let mut best = 0;
    for m in &meetings {
        let s = m[0];
        let running = meetings.iter().filter(|x| x[0] <= s && s < x[1]).count() as i64;
        best = best.max(running);
    }
    best
}
