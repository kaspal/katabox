pub fn kinship_distances(parent: Vec<i64>, pairs: Vec<Vec<i64>>) -> Vec<i64> {
    let n = parent.len();
    let mut children = vec![Vec::new(); n];
    let mut root = 0;
    for (i, &p) in parent.iter().enumerate() {
        if p == -1 {
            root = i;
        } else {
            children[p as usize].push(i);
        }
    }
    let mut depth = vec![0i64; n];
    let mut order = vec![root];
    let mut h = 0;
    while h < order.len() {
        let u = order[h];
        for &v in &children[u] {
            depth[v] = depth[u] + 1;
            order.push(v);
        }
        h += 1;
    }
    let mut levels = 1;
    while (1usize << levels) < n {
        levels += 1;
    }
    let mut up: Vec<Vec<usize>> = vec![parent.iter().map(|&p| if p == -1 { root } else { p as usize }).collect()];
    for k in 1..levels {
        let row: Vec<usize> = (0..n).map(|v| up[k - 1][up[k - 1][v]]).collect();
        up.push(row);
    }
    pairs
        .iter()
        .map(|q| {
            let (a, b) = (q[0] as usize, q[1] as usize);
            let (mut x, mut y) = if depth[a] >= depth[b] { (a, b) } else { (b, a) };
            let mut steps = depth[x] - depth[y];
            let mut k = 0;
            while steps > 0 {
                if steps & 1 == 1 {
                    x = up[k][x];
                }
                steps >>= 1;
                k += 1;
            }
            if x != y {
                for k in (0..levels).rev() {
                    if up[k][x] != up[k][y] {
                        x = up[k][x];
                        y = up[k][y];
                    }
                }
                x = up[0][x];
            }
            depth[a] + depth[b] - 2 * depth[x]
        })
        .collect()
}
