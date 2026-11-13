use std::collections::HashSet;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let mut seen = HashSet::with_capacity(n);
    let mut distinct = vec![0usize; n + 1];
    for p in 1..=n {
        seen.insert(tokens.next().unwrap());
        distinct[p] = seen.len();
    }
    let q = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let p = tokens.next().unwrap() as usize;
        writeln!(out, "{}", distinct[p]).unwrap();
    }
}
