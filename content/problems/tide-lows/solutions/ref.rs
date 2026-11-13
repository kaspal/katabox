use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let levels: Vec<i64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    // low[k][i]: the lowest of the 2^k readings starting at i.
    let mut low = vec![levels];
    let mut k = 1;
    while (1usize << k) <= n {
        let prev = &low[k - 1];
        let half = 1usize << (k - 1);
        let row: Vec<i64> = (0..=n - (1 << k)).map(|i| prev[i].min(prev[i + half])).collect();
        low.push(row);
        k += 1;
    }
    let q = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let l = tokens.next().unwrap() as usize - 1;
        let r = tokens.next().unwrap() as usize - 1;
        let len = r - l + 1;
        let k = (usize::BITS - 1 - len.leading_zeros()) as usize;
        let lowest = low[k][l].min(low[k][r + 1 - (1 << k)]);
        writeln!(out, "{lowest}").unwrap();
    }
}
