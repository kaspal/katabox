use std::io::{self, Read, Write};

// Correct but O(label length) per check: reads the packed label from its start, run by
// run, every time, until the running total reaches k.
fn char_at(label: &[u8], k: u64) -> char {
    let mut count = 0u64;
    let mut total = 0u64;
    for &b in label {
        if b.is_ascii_digit() {
            count = count * 10 + u64::from(b - b'0');
        } else {
            total += count;
            if total >= k {
                return b as char;
            }
            count = 0;
        }
    }
    panic!("k is past the end of the label");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let n_labels: usize = tokens.next().unwrap().parse().unwrap();
    let labels: Vec<&[u8]> = (0..n_labels).map(|_| tokens.next().unwrap().as_bytes()).collect();
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let i: usize = tokens.next().unwrap().parse().unwrap();
        let k: u64 = tokens.next().unwrap().parse().unwrap();
        writeln!(out, "{}", char_at(labels[i - 1], k)).unwrap();
    }
}
