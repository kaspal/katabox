// Correct but slow: walk the sorted prices for every budget, O(n·Q).
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let q = tokens.next().unwrap() as usize;
    let mut prices: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    prices.sort_unstable();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let budget = tokens.next().unwrap();
        let mut spent = 0;
        let mut count = 0;
        for &p in &prices {
            if spent + p > budget {
                break;
            }
            spent += p;
            count += 1;
        }
        writeln!(out, "{count}").unwrap();
    }
}
