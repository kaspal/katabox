// Correct but slow: collect each stretch's species, O(n) per question.
use std::collections::HashSet;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let ids: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    let q = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let l = tokens.next().unwrap() as usize;
        let r = tokens.next().unwrap() as usize;
        let seen: HashSet<u64> = ids[l - 1..r].iter().copied().collect();
        writeln!(out, "{}", seen.len()).unwrap();
    }
}
