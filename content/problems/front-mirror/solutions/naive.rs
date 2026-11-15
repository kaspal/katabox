// Correct but slow: check prefixes from the longest down, letter by letter, O(n²).
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim().as_bytes();
    let n = s.len();
    let mut length = 1;
    for cand in (1..=n).rev() {
        let (mut i, mut j) = (0, cand - 1);
        while i < j && s[i] == s[j] {
            i += 1;
            j -= 1;
        }
        if i >= j {
            length = cand;
            break;
        }
    }
    let mut out: Vec<u8> = s[length..].iter().rev().copied().collect();
    out.extend_from_slice(s);
    println!("{}", String::from_utf8(out).unwrap());
}
