use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn trapped_rain_grid(heights: Vec<Vec<i64>>) -> i64 {
    let (rows, cols) = (heights.len(), heights[0].len());
    let mut seen = vec![false; rows * cols];
    let mut heap = BinaryHeap::new();
    for r in 0..rows {
        for c in 0..cols {
            if r == 0 || r == rows - 1 || c == 0 || c == cols - 1 {
                seen[r * cols + c] = true;
                heap.push(Reverse((heights[r][c], r, c)));
            }
        }
    }
    let mut water = 0;
    while let Some(Reverse((level, r, c))) = heap.pop() {
        let moves = [(r + 1, c), (r.wrapping_sub(1), c), (r, c + 1), (r, c.wrapping_sub(1))];
        for (nr, nc) in moves {
            if nr >= rows || nc >= cols || seen[nr * cols + nc] {
                continue;
            }
            seen[nr * cols + nc] = true;
            let h = heights[nr][nc];
            water += (level - h).max(0);
            heap.push(Reverse((level.max(h), nr, nc)));
        }
    }
    water
}
