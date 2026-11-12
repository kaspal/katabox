fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

pub fn cheapest_network(n: i64, cables: Vec<Vec<i64>>) -> i64 {
    let n = n as usize;
    let mut parent: Vec<usize> = (0..n).collect();
    let mut sorted = cables;
    sorted.sort_unstable_by_key(|c| c[2]);
    let (mut total, mut laid) = (0i64, 0usize);
    for c in &sorted {
        if laid == n - 1 {
            break;
        }
        let (ra, rb) = (find(&mut parent, c[0] as usize), find(&mut parent, c[1] as usize));
        if ra != rb {
            parent[ra] = rb;
            total += c[2];
            laid += 1;
        }
    }
    if laid == n - 1 { total } else { -1 }
}
