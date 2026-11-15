use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim().as_bytes();
    // The longest palindrome at the start of s is the longest prefix of s that is also a
    // suffix of reversed(s). The prefix function of s + "#" + reversed(s) gives exactly
    // that at its last position ("#" stops a match from running across the middle).
    let reversed: Vec<u8> = s.iter().rev().copied().collect();
    let t: Vec<u8> = s.iter().chain(b"#").chain(&reversed).copied().collect();
    let mut pi = vec![0usize; t.len()];
    for i in 1..t.len() {
        let mut k = pi[i - 1];
        while k > 0 && t[i] != t[k] {
            k = pi[k - 1];
        }
        if t[i] == t[k] {
            k += 1;
        }
        pi[i] = k;
    }
    let keep = pi[t.len() - 1];
    let mut out = reversed[..s.len() - keep].to_vec();
    out.extend_from_slice(s);
    println!("{}", String::from_utf8(out).unwrap());
}
