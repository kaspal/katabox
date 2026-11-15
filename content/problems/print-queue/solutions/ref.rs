use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
    // A max-heap on (priority, earlier arrival, smaller number): the printer's next job.
    let mut waiting: BinaryHeap<(u64, Reverse<u64>, Reverse<usize>, u64)> = BinaryHeap::new();
    let mut order: Vec<String> = Vec::with_capacity(n);
    let (mut t, mut next) = (0u64, 0usize);
    while order.len() < n {
        if waiting.is_empty() && jobs[next].0 > t {
            t = jobs[next].0; // nothing to print: wait for the next job
        }
        while next < n && jobs[next].0 <= t {
            let (a, p, g, i) = jobs[next];
            waiting.push((p, Reverse(a), Reverse(i), g));
            next += 1;
        }
        let (_, _, Reverse(i), g) = waiting.pop().unwrap();
        order.push(i.to_string());
        t += g;
    }
    println!("{}", order.join(" "));
}
