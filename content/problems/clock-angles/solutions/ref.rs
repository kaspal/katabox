use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let t = tokens.next().unwrap();
        let h: i32 = t[..2].parse().unwrap();
        let m: i32 = t[3..].parse().unwrap();
        // In half-degrees, so everything stays whole: the hour hand is at 60·h + m,
        // the minute hand at 12·m.
        let mut gap = (60 * (h % 12) - 11 * m).abs();
        gap = gap.min(720 - gap);
        writeln!(out, "{}.{}", gap / 2, if gap % 2 == 1 { 5 } else { 0 }).unwrap();
    }
}
