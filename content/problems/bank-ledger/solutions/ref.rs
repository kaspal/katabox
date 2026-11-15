use std::collections::HashMap;
use std::io::{self, Read, Write};

struct Ledger<'a> {
    balance: HashMap<&'a str, i64>,
    holders: HashMap<i64, u32>, // balance → how many accounts hold it (for COUNT)
}

impl<'a> Ledger<'a> {
    fn put(&mut self, name: &'a str, value: Option<i64>) {
        if let Some(old) = self.balance.get(name) {
            *self.holders.get_mut(old).unwrap() -= 1;
        }
        match value {
            None => {
                self.balance.remove(name);
            }
            Some(v) => {
                self.balance.insert(name, v);
                *self.holders.entry(v).or_insert(0) += 1;
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let q: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut ledger = Ledger { balance: HashMap::new(), holders: HashMap::new() };
    // One undo log for all open transactions: (name, balance before). BEGIN remembers
    // where the log stood; ROLLBACK undoes back to there; COMMIT only forgets the mark,
    // so its changes now belong to the enclosing transaction.
    let mut log: Vec<(&str, Option<i64>)> = Vec::new();
    let mut marks: Vec<usize> = Vec::new();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for line in lines.take(q) {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        match parts[0] {
            "SET" | "ADD" => {
                let name = parts[1];
                let amount: i64 = parts[2].parse().unwrap();
                let before = ledger.balance.get(name).copied();
                let value = if parts[0] == "SET" { amount } else { before.unwrap_or(0) + amount };
                if !marks.is_empty() {
                    log.push((name, before));
                }
                ledger.put(name, Some(value));
            }
            "GET" => match ledger.balance.get(parts[1]) {
                Some(v) => writeln!(out, "{v}").unwrap(),
                None => writeln!(out, "NONE").unwrap(),
            },
            "COUNT" => {
                let x: i64 = parts[1].parse().unwrap();
                writeln!(out, "{}", ledger.holders.get(&x).copied().unwrap_or(0)).unwrap();
            }
            "BEGIN" => marks.push(log.len()),
            cmd => match marks.pop() {
                None => writeln!(out, "NO TRANSACTION").unwrap(),
                Some(mark) if cmd == "ROLLBACK" => {
                    while log.len() > mark {
                        let (name, old) = log.pop().unwrap();
                        ledger.put(name, old);
                    }
                }
                Some(_) => {
                    if marks.is_empty() {
                        log.clear(); // nothing can be rolled back any more
                    }
                }
            },
        }
    }
}
