use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let mut jobs: Vec<(u64, u64)> = (0..n)
        .map(|_| {
            let d = tokens.next().unwrap();
            let p = tokens.next().unwrap();
            (p, d)
        })
        .collect();
    jobs.sort_unstable_by(|a, b| b.cmp(a));
    // Best fee first, each on the latest free day by its deadline (only days 1..n matter).
    // free[d] points towards the latest free day ≤ d; day 0 means "none left". Taking a
    // day links it to the day before (union-find with path halving).
    let mut free: Vec<usize> = (0..=n).collect();
    let mut total: u64 = 0;
    for (fee, deadline) in jobs {
        let mut d = deadline.min(n as u64) as usize;
        while free[d] != d {
            free[d] = free[free[d]];
            d = free[d];
        }
        if d > 0 {
            free[d] = d - 1;
            total += fee;
        }
    }
    println!("{total}");
}
