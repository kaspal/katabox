pub fn lit_time(lamps: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n²): cut time at every start and end; a piece between two cuts is
    // lit when some lamp covers it, which is checked against every lamp.
    let mut cuts: Vec<i64> = lamps.iter().flat_map(|l| [l[0], l[1]]).collect();
    cuts.sort_unstable();
    cuts.dedup();
    let mut total = 0;
    for w in cuts.windows(2) {
        if lamps.iter().any(|l| l[0] <= w[0] && w[1] <= l[1]) {
            total += w[1] - w[0];
        }
    }
    total
}
