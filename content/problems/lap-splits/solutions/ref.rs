use std::io::{self, Read, Write};

fn seconds(lap: &str) -> u32 {
    let (m, s) = lap.split_once(':').unwrap();
    m.parse::<u32>().unwrap() * 60 + s.parse::<u32>().unwrap()
}

fn clock(total: u32) -> String {
    format!("{}:{:02}", total / 60, total % 60)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let t: usize = tokens.next().unwrap().parse().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let name = tokens.next().unwrap();
        let k: usize = tokens.next().unwrap().parse().unwrap();
        let laps: Vec<u32> = (0..k).map(|_| seconds(tokens.next().unwrap())).collect();
        let total: u32 = laps.iter().sum();
        let best = *laps.iter().min().unwrap();
        writeln!(out, "{name} {} {}", clock(total), clock(best)).unwrap();
    }
}
