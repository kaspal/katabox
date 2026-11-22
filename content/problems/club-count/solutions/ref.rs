pub fn club_count(n: i64, friendships: Vec<Vec<i64>>) -> Vec<i64> {
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut parent: Vec<usize> = (0..n as usize).collect();
    let mut clubs = n;
    let mut out = Vec::with_capacity(friendships.len());
    for f in &friendships {
        let ra = find(&mut parent, f[0] as usize);
        let rb = find(&mut parent, f[1] as usize);
        if ra != rb {
            parent[ra] = rb;
            clubs -= 1;
        }
        out.push(clubs);
    }
    out
}
