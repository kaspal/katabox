use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let label_count: usize = tokens.next().unwrap().parse().unwrap();
    // Per label: the end position of each run and its letter.
    let mut labels: Vec<(Vec<u64>, Vec<u8>)> = Vec::with_capacity(label_count);
    for _ in 0..label_count {
        let (mut ends, mut letters) = (Vec::new(), Vec::new());
        let (mut count, mut total) = (0u64, 0u64);
        for &b in tokens.next().unwrap().as_bytes() {
            if b.is_ascii_digit() {
                count = count * 10 + u64::from(b - b'0');
            } else {
                total += count;
                ends.push(total);
                letters.push(b);
                count = 0;
            }
        }
        labels.push((ends, letters));
    }
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let i: usize = tokens.next().unwrap().parse().unwrap();
        let k: u64 = tokens.next().unwrap().parse().unwrap();
        let (ends, letters) = &labels[i - 1];
        let r = ends.partition_point(|&e| e < k);
        writeln!(out, "{}", letters[r] as char).unwrap();
    }
}
