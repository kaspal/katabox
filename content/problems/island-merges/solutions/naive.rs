// Correct but slow: recount the islands over all land after every new square,
// O(Q · land).
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<usize>().unwrap());
    let r = tokens.next().unwrap();
    let c = tokens.next().unwrap();
    let q = tokens.next().unwrap();
    let width = c + 2;
    let mut land = vec![false; (r + 2) * width];
    let mut seen = vec![0u32; (r + 2) * width]; // the round a square was last seen in
    let mut cells: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for round in 1..=q as u32 {
        let cell = tokens.next().unwrap() * width + tokens.next().unwrap();
        if !land[cell] {
            land[cell] = true;
            cells.push(cell);
        }
        let mut islands = 0;
        for &start in &cells {
            if seen[start] == round {
                continue;
            }
            islands += 1;
            seen[start] = round;
            stack.push(start);
            while let Some(cur) = stack.pop() {
                for next in [cur + 1, cur - 1, cur + width, cur - width] {
                    if land[next] && seen[next] != round {
                        seen[next] = round;
                        stack.push(next);
                    }
                }
            }
        }
        writeln!(out, "{islands}").unwrap();
    }
}
