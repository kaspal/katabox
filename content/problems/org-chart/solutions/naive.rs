// Correct but slow: walk up from b for every question, O(depth) each.
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let mut boss = vec![0usize; n + 1];
    for b in boss.iter_mut().skip(2) {
        *b = tokens.next().unwrap();
    }
    let q = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        let mut x = b;
        while x != 1 && x != a {
            x = boss[x];
        }
        writeln!(out, "{}", if x == a && a != b { "YES" } else { "NO" }).unwrap();
    }
}
