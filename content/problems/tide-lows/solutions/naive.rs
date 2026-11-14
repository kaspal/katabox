// Correct but slow: scan every stretch, O(N·Q).
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let levels: Vec<i64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    let q = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let l = tokens.next().unwrap() as usize;
        let r = tokens.next().unwrap() as usize;
        let mut lowest = levels[l - 1];
        for &level in &levels[l - 1..r] {
            if level < lowest {
                lowest = level;
            }
        }
        writeln!(out, "{lowest}").unwrap();
    }
}
