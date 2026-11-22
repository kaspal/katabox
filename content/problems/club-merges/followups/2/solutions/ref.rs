pub fn clubs_with_undo(n: i64, ops: Vec<Vec<i64>>) -> Vec<i64> {
    let n = n as usize;
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size = vec![1usize; n];
    let mut history: Vec<Option<usize>> = Vec::new();
    let mut clubs = n as i64;
    let find = |parent: &[usize], mut x: usize| {
        while parent[x] != x {
            x = parent[x];
        }
        x
    };
    let mut out = Vec::with_capacity(ops.len());
    for op in &ops {
        if op[0] == 0 {
            if let Some(Some(rb)) = history.pop() {
                let ra = parent[rb];
                size[ra] -= size[rb];
                parent[rb] = rb;
                clubs += 1;
            }
        } else {
            let (mut ra, mut rb) = (find(&parent, op[1] as usize), find(&parent, op[2] as usize));
            if ra == rb {
                history.push(None);
            } else {
                if size[ra] < size[rb] {
                    std::mem::swap(&mut ra, &mut rb);
                }
                parent[rb] = ra;
                size[ra] += size[rb];
                clubs -= 1;
                history.push(Some(rb));
            }
        }
        out.push(clubs);
    }
    out
}
