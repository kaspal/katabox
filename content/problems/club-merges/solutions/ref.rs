pub fn clubs_left(n: i64, merges: Vec<Vec<i64>>) -> Vec<i64> {
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
    let mut clubs = n as i64;
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
                clubs -= 1;
            }
            clubs
        })
        .collect()
}
