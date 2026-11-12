pub fn cheapest_network(n: i64, cables: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n·m): grow the network from village 0, each time rescanning every
    // cable for the cheapest one that reaches a new village (a simple Prim's algorithm).
    let n = n as usize;
    let mut inside = vec![false; n];
    inside[0] = true;
    let mut total = 0;
    for _ in 1..n {
        let mut best: Option<(usize, usize, i64)> = None;
        for c in &cables {
            let (a, b) = (c[0] as usize, c[1] as usize);
            if inside[a] != inside[b] && best.is_none_or(|(_, _, bc)| c[2] < bc) {
                best = Some((a, b, c[2]));
            }
        }
        match best {
            Some((a, b, cost)) => {
                inside[a] = true;
                inside[b] = true;
                total += cost;
            }
            None => return -1,
        }
    }
    total
}
