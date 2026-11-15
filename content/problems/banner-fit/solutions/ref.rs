use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let r: usize = tokens.next().unwrap().parse().unwrap();
    let c: usize = tokens.next().unwrap().parse().unwrap();
    // height[x]: how many free squares stand in column x, ending at the current row. Each
    // row then is a skyline, and the best banner whose bottom edge is on this row is the
    // largest rectangle under it: a stack of columns of increasing height.
    let mut height = vec![0u64; c + 1]; // a 0 at the end empties the stack
    let mut stack: Vec<usize> = Vec::with_capacity(c + 1);
    let mut best = 0;
    for _ in 0..r {
        let row = tokens.next().unwrap().as_bytes();
        for x in 0..c {
            height[x] = if row[x] == b'.' { height[x] + 1 } else { 0 };
        }
        stack.clear();
        for x in 0..=c {
            while let Some(&top) = stack.last() {
                if height[top] < height[x] {
                    break;
                }
                stack.pop();
                let left = stack.last().map_or(0, |&l| l + 1);
                best = best.max(height[top] * (x - left) as u64);
            }
            stack.push(x);
        }
    }
    println!("{best}");
}
