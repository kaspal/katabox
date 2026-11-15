use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let mut team = vec![Vec::new(); n + 1];
    for i in 2..=n {
        team[tokens.next().unwrap()].push(i);
    }
    // Number the employees in depth-first order, with an explicit stack (a chart can be
    // 10⁵ levels deep: far too deep for recursion). first[x] is x's number; last[x] the
    // largest number in x's team, so the team is exactly first[x]..last[x].
    let mut first = vec![0u32; n + 1];
    let mut last = vec![0u32; n + 1];
    let mut clock = 0;
    let mut stack = vec![(1usize, false)];
    while let Some((x, done)) = stack.pop() {
        if done {
            last[x] = clock;
            continue;
        }
        clock += 1;
        first[x] = clock;
        stack.push((x, true));
        stack.extend(team[x].iter().map(|&y| (y, false)));
    }
    let q = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        let above = a != b && first[a] <= first[b] && first[b] <= last[a];
        writeln!(out, "{}", if above { "YES" } else { "NO" }).unwrap();
    }
}
