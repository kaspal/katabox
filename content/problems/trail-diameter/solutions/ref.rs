use std::io::{self, Read, Write};

// Iterative BFS: returns (the last junction reached, its distance), a farthest one.
fn farthest(adj: &[Vec<usize>], from: usize) -> (usize, usize) {
    let mut dist = vec![usize::MAX; adj.len()];
    let mut queue = Vec::with_capacity(adj.len());
    dist[from] = 0;
    queue.push(from);
    let mut head = 0;
    while head < queue.len() {
        let v = queue[head];
        head += 1;
        for &w in &adj[v] {
            if dist[w] == usize::MAX {
                dist[w] = dist[v] + 1;
                queue.push(w);
            }
        }
    }
    let last = queue[queue.len() - 1];
    (last, dist[last])
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<usize>().unwrap());
    let t = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let n = tokens.next().unwrap();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for _ in 0..n - 1 {
            let a = tokens.next().unwrap();
            let b = tokens.next().unwrap();
            adj[a].push(b);
            adj[b].push(a);
        }
        let (u, _) = farthest(&adj, 1);
        let (_, diameter) = farthest(&adj, u);
        writeln!(out, "{diameter}").unwrap();
    }
}
