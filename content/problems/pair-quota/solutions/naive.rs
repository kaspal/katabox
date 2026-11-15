// Correct but slow: every pair, O(n²) per case.
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let t = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let n = tokens.next().unwrap() as usize;
        let k = tokens.next().unwrap();
        let values: Vec<i64> = (0..n).map(|_| tokens.next().unwrap()).collect();
        let mut count: u64 = 0;
        for i in 0..n {
            for j in i + 1..n {
                if values[i] + values[j] <= k {
                    count += 1;
                }
            }
        }
        writeln!(out, "{count}").unwrap();
    }
}
