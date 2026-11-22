pub fn pond_fences(ice: Vec<String>) -> Vec<i64> {
    let grid: Vec<&[u8]> = ice.iter().map(|r| r.as_bytes()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut seen = vec![false; rows * cols];
    let mut fences = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != b'#' || seen[r * cols + c] {
                continue;
            }
            seen[r * cols + c] = true;
            let mut stack = vec![(r, c)];
            let mut edges = 0i64;
            while let Some((y, x)) = stack.pop() {
                let moves = [(y + 1, x), (y.wrapping_sub(1), x), (y, x + 1), (y, x.wrapping_sub(1))];
                for (ny, nx) in moves {
                    if ny < rows && nx < cols && grid[ny][nx] == b'#' {
                        if !seen[ny * cols + nx] {
                            seen[ny * cols + nx] = true;
                            stack.push((ny, nx));
                        }
                    } else {
                        edges += 1;
                    }
                }
            }
            fences.push(edges);
        }
    }
    fences.sort_unstable_by(|a, b| b.cmp(a));
    fences
}
