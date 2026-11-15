use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let t = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let n = tokens.next().unwrap() as usize;
        let k = tokens.next().unwrap();
        let mut values: Vec<i64> = (0..n).map(|_| tokens.next().unwrap()).collect();
        values.sort_unstable();
        // Two pointers on the sorted values: if the smallest and the largest fit, the
        // smallest fits with everything in between too.
        let mut count: u64 = 0;
        let (mut i, mut j) = (0, n - 1);
        while i < j {
            if values[i] + values[j] <= k {
                count += (j - i) as u64;
                i += 1;
            } else {
                j -= 1;
            }
        }
        writeln!(out, "{count}").unwrap();
    }
}
