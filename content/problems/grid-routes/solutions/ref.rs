pub fn grid_routes(floor: Vec<String>) -> i64 {
    const MOD: i64 = 1_000_000_007;
    let grid: Vec<&[u8]> = floor.iter().map(|r| r.as_bytes()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut ways = vec![0i64; cols + 1];
    for r in (0..rows).rev() {
        for c in (0..cols).rev() {
            ways[c] = if grid[r][c] == b'#' {
                0
            } else if r == rows - 1 && c == cols - 1 {
                1
            } else {
                (ways[c] + ways[c + 1]) % MOD
            };
        }
    }
    ways[0]
}
