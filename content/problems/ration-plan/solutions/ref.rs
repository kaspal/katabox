use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let q = tokens.next().unwrap() as usize;
    let mut prices: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    prices.sort_unstable();
    // total[k - 1]: the cost of the k cheapest rations. It only grows, so the most
    // rations a budget buys is the number of totals within it.
    let total: Vec<u64> = prices
        .iter()
        .scan(0, |sum, &p| {
            *sum += p;
            Some(*sum)
        })
        .collect();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let budget = tokens.next().unwrap();
        writeln!(out, "{}", total.partition_point(|&t| t <= budget)).unwrap();
    }
}
