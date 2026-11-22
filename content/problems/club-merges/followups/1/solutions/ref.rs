fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

pub fn biggest_club(n: i64, merges: Vec<Vec<i64>>) -> Vec<i64> {
    let n = n as usize;
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size = vec![1i64; n];
    let mut biggest = 1;
    merges
        .iter()
        .map(|m| {
            let (mut ra, mut rb) = (find(&mut parent, m[0] as usize), find(&mut parent, m[1] as usize));
            if ra != rb {
                if size[ra] < size[rb] {
                    std::mem::swap(&mut ra, &mut rb);
                }
                parent[rb] = ra;
                size[ra] += size[rb];
                biggest = biggest.max(size[ra]);
            }
            biggest
        })
        .collect()
}
