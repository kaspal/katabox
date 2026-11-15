use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let t = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let n = tokens.next().unwrap() as usize;
        let mut heights: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
        // The middle singer, or the lower of the two middle ones when n is even.
        let (_, median, _) = heights.select_nth_unstable((n - 1) / 2);
        writeln!(out, "{median}").unwrap();
    }
}
