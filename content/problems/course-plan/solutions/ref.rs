use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut unlocks = vec![Vec::new(); n + 1];
    let mut missing = vec![0u32; n + 1]; // prerequisites not passed yet (a repeated rule counts twice)
    for _ in 0..m {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        unlocks[a].push(b);
        missing[b] += 1;
    }
    // Kahn's algorithm, one term at a time: a term is every course whose last missing
    // prerequisite was passed in the term before.
    let mut term: Vec<usize> = (1..=n).filter(|&c| missing[c] == 0).collect();
    let mut terms: Vec<Vec<usize>> = Vec::new();
    let mut passed = 0;
    while !term.is_empty() {
        term.sort_unstable();
        passed += term.len();
        let mut following = Vec::new();
        for &a in &term {
            for &b in &unlocks[a] {
                missing[b] -= 1;
                if missing[b] == 0 {
                    following.push(b);
                }
            }
        }
        terms.push(std::mem::replace(&mut term, following));
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    if passed < n {
        writeln!(out, "IMPOSSIBLE").unwrap();
        return;
    }
    writeln!(out, "{}", terms.len()).unwrap();
    for t in &terms {
        let line: Vec<String> = t.iter().map(|c| c.to_string()).collect();
        writeln!(out, "{}", line.join(" ")).unwrap();
    }
}
