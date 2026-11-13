pub fn most_talks(talks: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n²): sorted by end, best[i] = the most talks ending with talk i,
    // scanning every earlier talk.
    let mut t = talks;
    t.sort_by_key(|x| x[1]);
    let ends: Vec<i64> = t.iter().map(|x| x[1]).collect();
    let mut best = vec![1i64; t.len()];
    for i in 0..t.len() {
        let start = t[i][0];
        let mut b = 1;
        for j in 0..i {
            if ends[j] <= start && best[j] + 1 > b {
                b = best[j] + 1;
            }
        }
        best[i] = b;
    }
    best.into_iter().max().unwrap()
}
