use std::collections::HashMap;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (first, text) = input.split_once('\n').unwrap_or((input.as_str(), ""));
    let k: usize = first.trim().parse().unwrap();
    let mut counts: HashMap<&str, u64> = HashMap::new();
    for word in text.split(|c: char| !c.is_ascii_lowercase()).filter(|w| !w.is_empty()) {
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut ranked: Vec<(&str, u64)> = counts.into_iter().collect();
    ranked.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let mut out = io::BufWriter::new(io::stdout().lock());
    for &(word, count) in ranked.iter().take(k) {
        writeln!(out, "{word} {count}").unwrap();
    }
}
