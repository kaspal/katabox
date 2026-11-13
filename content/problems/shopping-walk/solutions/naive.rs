pub fn shortest_walk(stalls: String, need: String) -> i64 {
    // Correct but O(n²): grow a stretch from every start until it covers the list.
    let mut want = [0usize; 26];
    for b in need.bytes() {
        want[(b - b'a') as usize] += 1;
    }
    let s: Vec<usize> = stalls.bytes().map(|b| (b - b'a') as usize).collect();
    let mut best = usize::MAX;
    for i in 0..s.len() {
        let mut have = [0usize; 26];
        let mut missing = need.len();
        for (j, &c) in s.iter().enumerate().skip(i) {
            if have[c] < want[c] {
                missing -= 1;
            }
            have[c] += 1;
            if missing == 0 {
                best = best.min(j - i + 1);
                break;
            }
        }
    }
    if best == usize::MAX {
        -1
    } else {
        best as i64
    }
}
