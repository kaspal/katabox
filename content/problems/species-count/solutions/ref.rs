use std::collections::HashMap;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let ids: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    let q = tokens.next().unwrap() as usize;
    let mut by_end = vec![Vec::new(); n + 1];
    let mut starts = vec![0usize; q];
    for (k, start) in starts.iter_mut().enumerate() {
        *start = tokens.next().unwrap() as usize;
        by_end[tokens.next().unwrap() as usize].push(k);
    }
    // Offline, by right end: while the log is read up to photo r, mark only the latest
    // photo of each species. Then the species of l..r are the marks in l..r. A Fenwick
    // tree keeps the marks' prefix counts.
    let mut tree = vec![0i32; n + 1];
    let add = |tree: &mut Vec<i32>, at: usize, delta: i32| {
        let mut i = at;
        while i <= n {
            tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    };
    let prefix = |tree: &Vec<i32>, at: usize| {
        let (mut i, mut total) = (at, 0);
        while i > 0 {
            total += tree[i];
            i -= i & i.wrapping_neg();
        }
        total
    };
    let mut latest: HashMap<u64, usize> = HashMap::new();
    let mut answer = vec![0i32; q];
    for r in 1..=n {
        if let Some(before) = latest.insert(ids[r - 1], r) {
            add(&mut tree, before, -1);
        }
        add(&mut tree, r, 1);
        for &k in &by_end[r] {
            answer[k] = prefix(&tree, r) - prefix(&tree, starts[k] - 1);
        }
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    for a in answer {
        writeln!(out, "{a}").unwrap();
    }
}
