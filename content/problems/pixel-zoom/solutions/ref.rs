use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let head: Vec<usize> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();
    let (r, k) = (head[0], head[2]);
    let mut out = io::BufWriter::new(io::stdout().lock());
    for row in lines.take(r) {
        let wide: String = row.trim().chars().flat_map(|ch| std::iter::repeat_n(ch, k)).collect();
        for _ in 0..k {
            writeln!(out, "{wide}").unwrap();
        }
    }
}
