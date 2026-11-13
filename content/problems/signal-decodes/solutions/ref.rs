use std::io::{self, Read, Write};

const MOD: u64 = 1_000_003;

fn ways(s: &[u8]) -> u64 {
    // before = ways of the prefix one digit shorter, last = ways of the current prefix.
    let mut before = 1u64;
    let mut last = u64::from(s[0] != b'0');
    for i in 1..s.len() {
        let (c, p) = (s[i], s[i - 1]);
        let mut cur = if c != b'0' { last } else { 0 };
        if p == b'1' || (p == b'2' && c <= b'6') {
            cur += before; // "1x" or "20".."26"
        }
        before = last;
        last = cur % MOD;
    }
    last
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let t: usize = tokens.next().unwrap().parse().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let msg = tokens.next().unwrap().as_bytes();
        writeln!(out, "{}", ways(&msg[1..])).unwrap();
    }
}
