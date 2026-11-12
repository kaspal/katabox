pub fn ring_length(n: i64, roads: Vec<Vec<i64>>) -> i64 {
    let n = n as usize;
    let mut near: Vec<Vec<usize>> = vec![Vec::new(); n];
    for r in &roads {
        let (a, b) = (r[0] as usize, r[1] as usize);
        near[a].push(b);
        near[b].push(a);
    }
    let mut degree: Vec<usize> = near.iter().map(|x| x.len()).collect();
    let mut removed = vec![false; n];
    let mut queue: Vec<usize> = (0..n).filter(|&v| degree[v] == 1).collect();
    let mut h = 0;
    while h < queue.len() {
        let v = queue[h];
        h += 1;
        removed[v] = true;
        for &u in &near[v] {
            if !removed[u] {
                degree[u] -= 1;
                if degree[u] == 1 {
                    queue.push(u);
                }
            }
        }
    }
    (n - h) as i64
}
