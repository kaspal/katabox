use std::collections::HashSet;
use std::io::{self, Read, Write};

// Correct but O(N·Q): recounts the distinct ids of each prefix from scratch.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let scans: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    let q = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let p = tokens.next().unwrap() as usize;
        let mut seen = HashSet::new();
        for &id in &scans[..p] {
            seen.insert(id);
        }
        writeln!(out, "{}", seen.len()).unwrap();
    }
}
