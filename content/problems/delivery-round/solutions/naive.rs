pub fn shortest_round(dist: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n!): try every order of the stops (lexicographic permutations).
    let n = dist.len();
    let mut order: Vec<usize> = (1..n).collect();
    let mut best = i64::MAX;
    loop {
        let mut total = 0;
        let mut at = 0;
        for &j in &order {
            total += dist[at][j];
            at = j;
        }
        best = best.min(total + dist[at][0]);
        // Next permutation.
        let Some(i) = (1..order.len()).rev().find(|&i| order[i - 1] < order[i]) else {
            break;
        };
        let k = (i..order.len()).rev().find(|&k| order[k] > order[i - 1]).unwrap();
        order.swap(i - 1, k);
        order[i..].reverse();
    }
    best
}
