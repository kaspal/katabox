use std::collections::VecDeque;

pub fn two_teams(n: i64, rivalries: Vec<Vec<i64>>) -> bool {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for e in &rivalries {
        adj[e[0] as usize].push(e[1] as usize);
        adj[e[1] as usize].push(e[0] as usize);
    }
    let mut colour = vec![-1i8; n];
    let mut q = VecDeque::new();
    for s in 0..n {
        if colour[s] != -1 {
            continue;
        }
        colour[s] = 0;
        q.push_back(s);
        while let Some(x) = q.pop_front() {
            for &y in &adj[x] {
                if colour[y] == -1 {
                    colour[y] = 1 - colour[x];
                    q.push_back(y);
                } else if colour[y] == colour[x] {
                    return false;
                }
            }
        }
    }
    true
}
