pub fn fewest_blocks(plays: Vec<i64>, requests: Vec<Vec<i64>>) -> Vec<i64> {
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
    let mut next = vec![n; n + 1];
    let mut r = 0usize;
    for i in 0..n {
        while r < n && start[r] <= i {
            r += 1;
        }
        next[i] = r;
    }
    let mut jump: Vec<Vec<usize>> = vec![next];
    while (1usize << jump.len()) <= n {
        let prev = jump.last().unwrap();
        let row: Vec<usize> = (0..=n).map(|i| prev[prev[i]]).collect();
        jump.push(row);
    }
    requests
        .iter()
        .map(|q| {
            let (a, b) = (q[0] as usize, q[1] as usize);
            let (mut pos, mut blocks) = (a, 1i64);
            for k in (0..jump.len()).rev() {
                if jump[k][pos] <= b {
                    pos = jump[k][pos];
                    blocks += 1i64 << k;
                }
            }
            blocks
        })
        .collect()
}
