use std::io::{self, Read, Write};

const MOD: u64 = 1_000_003;

// Correct but exponential: the plain "one digit or two digits" recursion without memo,
// run with an explicit stack (so long messages cannot overflow the call stack). It
// reaches the end once per reading, so it visits every reading one by one.
fn ways(s: &[u8]) -> u64 {
    let n = s.len();
    let mut count = 0u64;
    let mut stack = vec![0usize];
    while let Some(i) = stack.pop() {
        if i == n {
            count = (count + 1) % MOD;
            continue;
        }
        let c = s[i];
        if c == b'0' {
            continue; // a piece never starts with 0
        }
        stack.push(i + 1);
        if i + 1 < n && (c == b'1' || (c == b'2' && s[i + 1] <= b'6')) {
            stack.push(i + 2);
        }
    }
    count
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
