pub fn ring_length(n: i64, roads: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n²): for every road, search for another path between its villages.
    let n = n as usize;
    let mut near: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (k, r) in roads.iter().enumerate() {
        near[r[0] as usize].push((r[1] as usize, k));
        near[r[1] as usize].push((r[0] as usize, k));
    }
    let mut on_loop = 0;
    for (k, r) in roads.iter().enumerate() {
        let (a, b) = (r[0] as usize, r[1] as usize);
        let mut seen = vec![false; n];
        seen[a] = true;
        let mut stack = vec![a];
        while let Some(v) = stack.pop() {
            for &(u, road) in &near[v] {
                if road != k && !seen[u] {
                    seen[u] = true;
                    stack.push(u);
                }
            }
        }
        if seen[b] {
            on_loop += 1;
        }
    }
    on_loop
}
