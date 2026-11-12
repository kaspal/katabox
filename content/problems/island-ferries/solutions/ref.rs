pub fn groups(n: i64, links: Vec<Vec<i64>>) -> i64 {
    let n = n as usize;
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size = vec![1usize; n];
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut merges = 0;
    for l in &links {
        let mut ra = find(&mut parent, l[0] as usize);
        let mut rb = find(&mut parent, l[1] as usize);
        if ra == rb {
            continue;
        }
        if size[ra] < size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        parent[rb] = ra;
        size[ra] += size[rb];
        merges += 1;
    }
    (n - merges) as i64
}
