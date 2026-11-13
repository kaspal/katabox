pub fn shortest_walk(stalls: String, need: String) -> i64 {
    let mut want = [0usize; 26];
    for b in need.bytes() {
        want[(b - b'a') as usize] += 1;
    }
    let s: Vec<usize> = stalls.bytes().map(|b| (b - b'a') as usize).collect();
    let mut have = [0usize; 26];
    let (mut missing, mut left, mut best) = (need.len(), 0, usize::MAX);
    for (right, &c) in s.iter().enumerate() {
        if have[c] < want[c] {
            missing -= 1;
        }
        have[c] += 1;
        while missing == 0 {
            best = best.min(right - left + 1);
            let d = s[left];
            have[d] -= 1;
            if have[d] < want[d] {
                missing += 1;
            }
            left += 1;
        }
    }
    if best == usize::MAX {
        -1
    } else {
        best as i64
    }
}
