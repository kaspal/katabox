use std::collections::HashSet;
use std::io::{self, Read, Write};

// Correct but O(N·Q): for each question, scans every score and collects the distinct
// ones above it.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let scores: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    let q = tokens.next().unwrap() as usize;
    let mut answers = Vec::with_capacity(q);
    for _ in 0..q {
        let x = tokens.next().unwrap();
        if !scores.contains(&x) {
            answers.push(0);
        } else {
            let above: HashSet<u64> = scores.iter().copied().filter(|&s| s > x).collect();
            answers.push(above.len() + 1);
        }
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    let line: Vec<String> = answers.iter().map(|a| a.to_string()).collect();
    writeln!(out, "{}", line.join(" ")).unwrap();
}
