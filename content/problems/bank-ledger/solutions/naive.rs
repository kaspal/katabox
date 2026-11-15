// Correct but slow: BEGIN copies the whole ledger and COUNT scans it, O(Q · accounts).
use std::collections::HashMap;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let q: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut ledger: HashMap<String, i64> = HashMap::new();
    let mut saved: Vec<HashMap<String, i64>> = Vec::new();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for line in lines.take(q) {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        match parts[0] {
            "SET" => {
                ledger.insert(parts[1].to_string(), parts[2].parse().unwrap());
            }
            "ADD" => {
                *ledger.entry(parts[1].to_string()).or_insert(0) += parts[2].parse::<i64>().unwrap();
            }
            "GET" => match ledger.get(parts[1]) {
                Some(v) => writeln!(out, "{v}").unwrap(),
                None => writeln!(out, "NONE").unwrap(),
            },
            "COUNT" => {
                let x: i64 = parts[1].parse().unwrap();
                writeln!(out, "{}", ledger.values().filter(|&&v| v == x).count()).unwrap();
            }
            "BEGIN" => saved.push(ledger.clone()),
            cmd => match saved.pop() {
                None => writeln!(out, "NO TRANSACTION").unwrap(),
                Some(old) => {
                    if cmd == "ROLLBACK" {
                        ledger = old;
                    }
                }
            },
        }
    }
}
