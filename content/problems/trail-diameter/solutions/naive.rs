use std::io::{self, Read, Write};

// Correct but O(N²): a BFS from every junction, keeping the largest distance seen.
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
        let mut best = 0;
        let mut dist = vec![usize::MAX; n + 1];
        let mut queue = Vec::with_capacity(n);
        for start in 1..=n {
            dist.fill(usize::MAX);
            queue.clear();
            dist[start] = 0;
            queue.push(start);
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
            best = best.max(dist[queue[queue.len() - 1]]);
        }
        writeln!(out, "{best}").unwrap();
    }
}
