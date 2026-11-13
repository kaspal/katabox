// Correct but slow: walk the departures of the line with the longest period and stop at the
// first one every other line shares, O(lcm / max m · n) per timetable.
use std::io::{self, Read, Write};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let t = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let n = tokens.next().unwrap();
        let lines: Vec<(u64, u64)> =
            (0..n).map(|_| (tokens.next().unwrap(), tokens.next().unwrap())).collect();
        let mut total = 1u64;
        for &(_, m) in &lines {
            total = total / gcd(total, m) * m;
        }
        let &(first, period) = lines.iter().max_by_key(|line| line.1).unwrap();
        let mut answer = -1i64;
        let mut minute = first;
        while minute < total {
            if lines.iter().all(|&(r, m)| minute % m == r) {
                answer = minute as i64;
                break;
            }
            minute += period;
        }
        writeln!(out, "{answer}").unwrap();
    }
}
