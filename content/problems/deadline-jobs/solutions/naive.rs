// Correct but slow: best fee first, searching back day by day for a free day, O(n²).
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
    let mut taken = vec![false; n + 1];
    let mut total: u64 = 0;
    for (fee, deadline) in jobs {
        let mut day = deadline.min(n as u64) as usize;
        while day > 0 && taken[day] {
            day -= 1;
        }
        if day > 0 {
            taken[day] = true;
            total += fee;
        }
    }
    println!("{total}");
}
