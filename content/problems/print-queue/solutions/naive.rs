// Correct but slow: a plain list of waiting jobs, scanned in full for every pick, O(n²).
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    // (arrival, priority, pages, number)
    let mut jobs: Vec<(u64, u64, u64, usize)> = (0..n)
        .map(|i| {
            let a = tokens.next().unwrap();
            let p = tokens.next().unwrap();
            let g = tokens.next().unwrap();
            (a, p, g, i + 1)
        })
        .collect();
    jobs.sort_unstable_by_key(|&(a, _, _, i)| (a, i));
    let mut waiting: Vec<(u64, u64, u64, usize)> = Vec::new();
    let mut order: Vec<String> = Vec::with_capacity(n);
    let (mut t, mut next) = (0u64, 0usize);
    while order.len() < n {
        if waiting.is_empty() && jobs[next].0 > t {
            t = jobs[next].0;
        }
        while next < n && jobs[next].0 <= t {
            waiting.push(jobs[next]);
            next += 1;
        }
        let mut best = 0;
        for k in 1..waiting.len() {
            let (a, p, _, i) = waiting[k];
            let (ba, bp, _, bi) = waiting[best];
            if (std::cmp::Reverse(p), a, i) < (std::cmp::Reverse(bp), ba, bi) {
                best = k;
            }
        }
        let (_, _, g, i) = waiting.swap_remove(best);
        order.push(i.to_string());
        t += g;
    }
    println!("{}", order.join(" "));
}
