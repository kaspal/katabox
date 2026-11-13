// Correct but slow: every rectangle. For each pair of rows, keep running column sums, then
// for each left column extend the right edge with a running total: O(R²·C²).
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let rows = tokens.next().unwrap() as usize;
    let cols = tokens.next().unwrap() as usize;
    let grid: Vec<i64> = tokens.take(rows * cols).collect();
    let mut best = i64::MIN;
    let mut col = vec![0i64; cols];
    for top in 0..rows {
        col.fill(0);
        for bottom in top..rows {
            for j in 0..cols {
                col[j] += grid[bottom * cols + j];
            }
            for left in 0..cols {
                let mut total = 0i64;
                for right in left..cols {
                    total += col[right];
                    if total > best {
                        best = total;
                    }
                }
            }
        }
    }
    println!("{best}");
}
