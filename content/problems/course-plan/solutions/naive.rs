// Correct but slow: every term, check every course left, O(terms · (n + m)).
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut needs = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        needs[b].push(a);
    }
    let mut passed = vec![false; n + 1];
    let mut left = n;
    let mut terms: Vec<Vec<usize>> = Vec::new();
    while left > 0 {
        let ready: Vec<usize> = (1..=n)
            .filter(|&c| !passed[c] && needs[c].iter().all(|&a| passed[a]))
            .collect();
        if ready.is_empty() {
            println!("IMPOSSIBLE");
            return;
        }
        for &c in &ready {
            passed[c] = true;
        }
        left -= ready.len();
        terms.push(ready);
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(out, "{}", terms.len()).unwrap();
    for t in &terms {
        let line: Vec<String> = t.iter().map(|c| c.to_string()).collect();
        writeln!(out, "{}", line.join(" ")).unwrap();
    }
}
