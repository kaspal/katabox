// Correct but slow: a fresh Dijkstra from the depot for every question (stopping when the
// town is reached), O(Q·(N + M) log N).
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read, Write};

fn search(adj: &[Vec<(usize, u64)>], target: usize) -> i64 {
    let mut dist = vec![u64::MAX; adj.len()];
    dist[1] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u64, 1usize)));
    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        if u == target {
            return d as i64;
        }
        for &(v, w) in &adj[u] {
            if d + w < dist[v] {
                dist[v] = d + w;
                heap.push(Reverse((d + w, v)));
            }
        }
    }
    -1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut adj: Vec<Vec<(usize, u64)>> = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        let w = tokens.next().unwrap() as u64;
        adj[a].push((b, w));
        adj[b].push((a, w));
    }
    let q = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let t = tokens.next().unwrap();
        writeln!(out, "{}", search(&adj, t)).unwrap();
    }
}
