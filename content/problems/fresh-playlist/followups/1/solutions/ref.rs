pub fn fresh_in_range(plays: Vec<i64>, requests: Vec<Vec<i64>>) -> Vec<i64> {
    use std::collections::HashMap;
    let n = plays.len();
    let mut start = vec![0usize; n];
    let mut last: HashMap<i64, usize> = HashMap::new();
    let mut left = 0usize;
    for (r, &song) in plays.iter().enumerate() {
        if let Some(&p) = last.get(&song) {
            if p >= left {
                left = p + 1;
            }
        }
        last.insert(song, r);
        start[r] = left;
    }
    let mut table: Vec<Vec<usize>> = vec![(0..n).map(|r| r - start[r] + 1).collect()];
    let mut j = 1;
    while (1usize << j) <= n {
        let half = 1usize << (j - 1);
        let prev = &table[j - 1];
        let row: Vec<usize> = (0..n - (1usize << j) + 1)
            .map(|i| prev[i].max(prev[i + half]))
            .collect();
        table.push(row);
        j += 1;
    }
    requests
        .iter()
        .map(|q| {
            let (a, b) = (q[0] as usize, q[1] as usize);
            let (mut lo, mut hi) = (a, b + 1);
            while lo < hi {
                let mid = (lo + hi) / 2;
                if start[mid] >= a {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            let mut best = lo - a;
            if lo <= b {
                let k = (usize::BITS - 1 - (b - lo + 1).leading_zeros()) as usize;
                best = best.max(table[k][lo]).max(table[k][b + 1 - (1usize << k)]);
            }
            best as i64
        })
        .collect()
}
