pub fn common_boss(boss: Vec<i64>, pairs: Vec<Vec<i64>>) -> Vec<i64> {
    let n = boss.len();
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut president = 0;
    for (v, &b) in boss.iter().enumerate() {
        if b < 0 {
            president = v;
        } else {
            children[b as usize].push(v);
        }
    }
    let mut depth = vec![0u32; n];
    let mut order = vec![president];
    let mut h = 0;
    while h < order.len() {
        let v = order[h];
        h += 1;
        for &c in &children[v] {
            depth[c] = depth[v] + 1;
            order.push(c);
        }
    }
    let mut levels = 1;
    while (1usize << levels) < n {
        levels += 1;
    }
    let mut up: Vec<Vec<usize>> = vec![boss.iter().map(|&b| if b < 0 { president } else { b as usize }).collect()];
    for k in 1..levels {
        let prev = &up[k - 1];
        let cur: Vec<usize> = (0..n).map(|v| prev[prev[v]]).collect();
        up.push(cur);
    }
    pairs
        .iter()
        .map(|p| {
            let (mut a, mut b) = (p[0] as usize, p[1] as usize);
            if depth[a] < depth[b] {
                std::mem::swap(&mut a, &mut b);
            }
            let mut d = depth[a] - depth[b];
            let mut k = 0;
            while d > 0 {
                if d & 1 == 1 {
                    a = up[k][a];
                }
                d >>= 1;
                k += 1;
            }
            if a == b {
                return a as i64;
            }
            for k in (0..levels).rev() {
                if up[k][a] != up[k][b] {
                    a = up[k][a];
                    b = up[k][b];
                }
            }
            up[0][a] as i64
        })
        .collect()
}
