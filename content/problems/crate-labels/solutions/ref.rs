use std::collections::BTreeMap;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.split_ascii_whitespace();
    let n: usize = lines.next().unwrap().parse().unwrap();
    // A BTreeMap keeps the names in alphabetical (byte) order.
    let mut total: BTreeMap<&str, u64> = BTreeMap::new();
    for label in lines.take(n) {
        for entry in label.split(';') {
            let (name, qty) = entry.split_once(':').unwrap();
            *total.entry(name).or_insert(0) += qty.parse::<u64>().unwrap();
        }
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    for (name, sum) in total {
        writeln!(out, "{name} {sum}").unwrap();
    }
}
