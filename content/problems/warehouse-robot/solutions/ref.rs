use std::collections::VecDeque;

pub fn shortest_route(grid: Vec<Vec<i64>>) -> i64 {
    let rows = grid.len();
    let cols = grid[0].len();
    if grid[0][0] == 1 || grid[rows - 1][cols - 1] == 1 {
        return -1;
    }
    let mut dist = vec![vec![-1i64; cols]; rows];
    dist[0][0] = 0;
    let mut queue = VecDeque::from([(0usize, 0usize)]);
    while let Some((r, c)) = queue.pop_front() {
        if r == rows - 1 && c == cols - 1 {
            return dist[r][c];
        }
        let moves: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dr, dc) in moves {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == 0 && dist[nr][nc] == -1 {
                dist[nr][nc] = dist[r][c] + 1;
                queue.push_back((nr, nc));
            }
        }
    }
    -1
}
