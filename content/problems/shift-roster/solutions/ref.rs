use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let n: u64 = tokens.next().unwrap().parse().unwrap();
    let mut roster: Vec<(&str, u64)> = Vec::new();
    for _ in 0..n {
        let name = tokens.next().unwrap();
        let hours: u64 = tokens.next().unwrap().parse().unwrap();
        roster.push((name, hours));
    }
    let total: u64 = roster.iter().map(|&(_, hours)| hours).sum();
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(out, "{total}").unwrap();
    let mut anyone = false;
    for &(name, hours) in &roster {
        // hours > total / n, in whole numbers
        if hours * n > total {
            writeln!(out, "{name} {hours}").unwrap();
            anyone = true;
        }
    }
    if !anyone {
        writeln!(out, "none").unwrap();
    }
}
