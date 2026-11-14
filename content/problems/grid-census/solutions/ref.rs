use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let r: usize = tokens.next().unwrap().parse().unwrap();
    let c: usize = tokens.next().unwrap().parse().unwrap();
    let mut row_trees = vec![0u32; r];
    let mut col_trees = vec![0u32; c];
    for i in 0..r {
        let row = tokens.next().unwrap().as_bytes();
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'#' {
                row_trees[i] += 1;
                col_trees[j] += 1;
            }
        }
    }
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let kind = tokens.next().unwrap();
        let index: usize = tokens.next().unwrap().parse().unwrap();
        let trees = if kind == "row" { row_trees[index] } else { col_trees[index] };
        writeln!(out, "{trees}").unwrap();
    }
}
