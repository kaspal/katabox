use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap();
    let mut prev = tokens.next().unwrap();
    let mut total = prev;
    let (mut rises, mut run, mut longest) = (0u64, 1u64, 1u64);
    for _ in 1..n {
        let x = tokens.next().unwrap();
        total += x;
        if x == prev {
            run += 1;
            longest = longest.max(run);
        } else {
            if x > prev {
                rises += 1;
            }
            run = 1;
        }
        prev = x;
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(out, "{total} {rises} {longest}").unwrap();
}
