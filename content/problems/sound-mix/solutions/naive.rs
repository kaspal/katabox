// Correct but slow: from every start, extend while at most K sources, O(n²).
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let k = tokens.next().unwrap() as usize;
    let ids: Vec<u64> = (0..n).map(|_| tokens.next().unwrap()).collect();
    let mut count: u64 = 0;
    for i in 0..n {
        let mut seen = HashSet::new();
        for &id in &ids[i..] {
            seen.insert(id);
            if seen.len() > k {
                break;
            }
            if seen.len() == k {
                count += 1;
            }
        }
    }
    println!("{count}");
}
