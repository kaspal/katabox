pub fn mutual_reach(n: i64, links: Vec<Vec<i64>>) -> Vec<i64> {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    let mut radj = vec![Vec::new(); n];
    for e in &links {
        adj[e[0] as usize].push(e[1] as usize);
        radj[e[1] as usize].push(e[0] as usize);
    }
    let mut seen = vec![false; n];
    let mut order = Vec::with_capacity(n);
    for s in 0..n {
        if seen[s] {
            continue;
        }
        seen[s] = true;
        let mut stack = vec![(s, 0usize)];
        while let Some(&(x, i)) = stack.last() {
            if i < adj[x].len() {
                stack.last_mut().unwrap().1 += 1;
                let y = adj[x][i];
                if !seen[y] {
                    seen[y] = true;
                    stack.push((y, 0));
                }
            } else {
                stack.pop();
                order.push(x);
            }
        }
    }
    let mut group = vec![usize::MAX; n];
    let mut sizes = Vec::new();
    for &s in order.iter().rev() {
        if group[s] != usize::MAX {
            continue;
        }
        let id = sizes.len();
        group[s] = id;
        let mut count = 0i64;
        let mut stack = vec![s];
        while let Some(x) = stack.pop() {
            count += 1;
            for &y in &radj[x] {
                if group[y] == usize::MAX {
                    group[y] = id;
                    stack.push(y);
                }
            }
        }
        sizes.push(count);
    }
    sizes.sort();
    sizes
}
