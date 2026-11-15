// Correct but slow: the best plan ending with each offer, trying every earlier offer, O(n²).
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    // (end, start, pay), sorted by end
    let mut gigs: Vec<(i64, i64, i64)> = (0..n)
        .map(|_| {
            let s = tokens.next().unwrap();
            let e = tokens.next().unwrap();
            let p = tokens.next().unwrap();
            (e, s, p)
        })
        .collect();
    gigs.sort_unstable();
    // (total, -count) of the best plan ending with gig i
    let mut best_with: Vec<(i64, i64)> = Vec::with_capacity(n);
    for i in 0..n {
        let (_, start, pay) = gigs[i];
        let mut before = (0, 0);
        for k in 0..i {
            if gigs[k].0 <= start && best_with[k] > before {
                before = best_with[k];
            }
        }
        best_with.push((before.0 + pay, before.1 - 1));
    }
    let (total, minus_count) = best_with.into_iter().max().unwrap();
    println!("{total} {}", -minus_count);
}
