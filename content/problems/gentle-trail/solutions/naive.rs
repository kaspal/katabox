pub fn gentlest_climb(heights: Vec<Vec<i64>>) -> i64 {
    // Correct but slow: try every distinct step difficulty in increasing order, with a
    // fresh search of the whole grid for each one.
    let (rows, cols) = (heights.len(), heights[0].len());
    let mut diffs = vec![0i64];
    for r in 0..rows {
        for c in 0..cols {
            if r + 1 < rows {
                diffs.push((heights[r][c] - heights[r + 1][c]).abs());
            }
            if c + 1 < cols {
                diffs.push((heights[r][c] - heights[r][c + 1]).abs());
            }
        }
    }
    diffs.sort_unstable();
    diffs.dedup();
    for limit in diffs {
        let mut seen = vec![vec![false; cols]; rows];
        seen[0][0] = true;
        let mut stack = vec![(0usize, 0usize)];
        while let Some((r, c)) = stack.pop() {
            let mut next = Vec::with_capacity(4);
            if r > 0 {
                next.push((r - 1, c));
            }
            if r + 1 < rows {
                next.push((r + 1, c));
            }
            if c > 0 {
                next.push((r, c - 1));
            }
            if c + 1 < cols {
                next.push((r, c + 1));
            }
            for (nr, nc) in next {
                if !seen[nr][nc] && (heights[nr][nc] - heights[r][c]).abs() <= limit {
                    seen[nr][nc] = true;
                    stack.push((nr, nc));
                }
            }
        }
        if seen[rows - 1][cols - 1] {
            return limit;
        }
    }
    -1
}
