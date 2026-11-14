use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let mut distinct: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    // Distinct scores, highest first.
    distinct.sort_unstable_by(|a, b| b.cmp(a));
    distinct.dedup();
    let q = tokens.next().unwrap() as usize;
    let mut line = String::with_capacity(q * 6);
    for j in 0..q {
        let x = tokens.next().unwrap();
        let rank = match distinct.binary_search_by(|s| x.cmp(s)) {
            Ok(index) => index + 1,
            Err(_) => 0,
        };
        if j > 0 {
            line.push(' ');
        }
        line.push_str(&rank.to_string());
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(out, "{line}").unwrap();
}
