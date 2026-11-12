pub fn groups(n: i64, links: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n·m): give each island the smallest label among its ferry
    // neighbours, pass after pass, until nothing changes.
    let mut label: Vec<i64> = (0..n).collect();
    let mut changed = true;
    while changed {
        changed = false;
        for l in &links {
            let (a, b) = (l[0] as usize, l[1] as usize);
            let low = label[a].min(label[b]);
            if label[a] != low || label[b] != low {
                label[a] = low;
                label[b] = low;
                changed = true;
            }
        }
    }
    label.sort_unstable();
    label.dedup();
    label.len() as i64
}
