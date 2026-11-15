use std::io::{self, Read, Write};

fn minutes(t: &str) -> u32 {
    t[..2].parse::<u32>().unwrap() * 60 + t[3..].parse::<u32>().unwrap()
}

fn clock(m: u32) -> String {
    format!("{:02}:{:02}", m / 60, m % 60)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let t: usize = tokens.next().unwrap().parse().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let open = minutes(tokens.next().unwrap());
        let close = minutes(tokens.next().unwrap());
        let k: usize = tokens.next().unwrap().parse().unwrap();
        let mut slots: Vec<(u32, u32)> = (0..k)
            .map(|_| (minutes(tokens.next().unwrap()), minutes(tokens.next().unwrap())))
            .collect();
        slots.sort_unstable();
        // Sweep the slots by start time; `free_from` is where the free time begins. A gap
        // is only replaced by a strictly longer one, so the earliest longest gap wins.
        let (mut best_len, mut best_start) = (0, 0);
        let mut free_from = open;
        for (s, e) in slots {
            if s > free_from && s - free_from > best_len {
                best_len = s - free_from;
                best_start = free_from;
            }
            free_from = free_from.max(e);
        }
        if close > free_from && close - free_from > best_len {
            best_len = close - free_from;
            best_start = free_from;
        }
        if best_len > 0 {
            writeln!(out, "{}-{}", clock(best_start), clock(best_start + best_len)).unwrap();
        } else {
            writeln!(out, "BOOKED").unwrap();
        }
    }
}
