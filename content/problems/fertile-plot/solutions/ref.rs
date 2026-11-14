use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let mut rows = tokens.next().unwrap() as usize;
    let mut cols = tokens.next().unwrap() as usize;
    let mut grid: Vec<i64> = tokens.take(rows * cols).collect();
    if rows > cols {
        // Put the shorter side in the outer loops.
        let mut flipped = vec![0; rows * cols];
        for i in 0..rows {
            for j in 0..cols {
                flipped[j * rows + i] = grid[i * cols + j];
            }
        }
        grid = flipped;
        std::mem::swap(&mut rows, &mut cols);
    }
    let mut best = i64::MIN;
    let mut col = vec![0i64; cols];
    for top in 0..rows {
        col.fill(0);
        for bottom in top..rows {
            let row = &grid[bottom * cols..(bottom + 1) * cols];
            let mut cur = 0i64; // Kadane: the best run of columns ending at j
            for (c, &x) in col.iter_mut().zip(row) {
                *c += x;
                cur = if cur > 0 { cur + *c } else { *c };
                best = best.max(cur);
            }
        }
    }
    println!("{best}");
}
