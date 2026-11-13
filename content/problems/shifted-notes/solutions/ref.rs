use std::io::{self, Read, Write};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let newline = input.iter().position(|&b| b == b'\n').unwrap_or(input.len());
    let s: u8 = std::str::from_utf8(&input[..newline]).unwrap().trim().parse().unwrap();
    let start = (newline + 1).min(input.len());
    let decoded: Vec<u8> = input[start..]
        .iter()
        .map(|&b| match b {
            b'a'..=b'z' => b'a' + (b - b'a' + 26 - s) % 26,
            b'A'..=b'Z' => b'A' + (b - b'A' + 26 - s) % 26,
            _ => b,
        })
        .collect();
    let mut out = io::BufWriter::new(io::stdout().lock());
    out.write_all(&decoded).unwrap();
}
