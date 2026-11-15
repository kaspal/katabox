use std::collections::HashMap;
use std::io::{self, Read};

/// How many stretches have at most m different sources (a sliding window).
fn at_most(ids: &[u64], m: usize) -> u64 {
    if m == 0 {
        return 0;
    }
    let mut count: HashMap<u64, u32> = HashMap::new();
    let mut left = 0;
    let mut total: u64 = 0;
    for right in 0..ids.len() {
        *count.entry(ids[right]).or_insert(0) += 1;
        while count.len() > m {
            let y = ids[left];
            left += 1;
            let c = count.get_mut(&y).unwrap();
            *c -= 1;
            if *c == 0 {
                count.remove(&y);
            }
        }
        // Every stretch ending at `right` and starting at left..right qualifies.
        total += (right - left + 1) as u64;
    }
    total
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let k = tokens.next().unwrap() as usize;
    let ids: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    // Exactly K = at most K minus at most K − 1.
    println!("{}", at_most(&ids, k) - at_most(&ids, k - 1));
}
