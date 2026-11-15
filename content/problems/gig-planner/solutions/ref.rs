use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    // (end, start, pay), sorted by end
    let mut gigs: Vec<(i64, i64, i64)> = (0..n)
        .map(|_| {
            let s = tokens.next().unwrap();
            let e = tokens.next().unwrap();
            let p = tokens.next().unwrap();
            (e, s, p)
        })
        .collect();
    gigs.sort_unstable();
    let ends: Vec<i64> = gigs.iter().map(|g| g.0).collect();
    // best[i]: the best plan using only the first i offers (by end), as (total, -count):
    // a higher total wins, then fewer gigs. Either offer i is skipped, or it is taken
    // after the best plan of the offers that end by its start (a binary search).
    let mut best = vec![(0i64, 0i64); n + 1];
    for i in 0..n {
        let (_, start, pay) = gigs[i];
        let j = ends[..i].partition_point(|&e| e <= start);
        let take = (best[j].0 + pay, best[j].1 - 1);
        best[i + 1] = best[i].max(take);
    }
    println!("{} {}", best[n].0, -best[n].1);
}
