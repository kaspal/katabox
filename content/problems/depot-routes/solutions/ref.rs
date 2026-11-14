use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read, Write};

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
    let mut dist = vec![u64::MAX; n + 1];
    dist[1] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u64, 1usize)));
    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue; // a stale entry
        }
        for &(v, w) in &adj[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }
    let q = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let d = dist[tokens.next().unwrap()];
        if d == u64::MAX {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{d}").unwrap();
        }
    }
}
