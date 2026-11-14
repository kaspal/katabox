use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let days = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..days {
        let n = tokens.next().unwrap();
        let (mut total, mut highest) = (0, 0);
        for _ in 0..n {
            let price = tokens.next().unwrap();
            total += price;
            highest = highest.max(price);
        }
        writeln!(out, "{n} {total} {highest}").unwrap();
    }
}
