pub fn cheapest_crossing(tolls: Vec<Vec<i64>>) -> i64 {
    // Correct but slow: sweep every state in row order, relaxing all neighbours,
    // until a whole sweep changes nothing (Bellman–Ford style).
    let (rows, cols) = (tolls.len(), tolls[0].len());
    let mut cost = vec![vec![vec![i64::MAX; cols]; rows]; 2];
    cost[0][0][0] = tolls[0][0];
    cost[1][0][0] = tolls[0][0] / 2;
    let mut changed = true;
    while changed {
        changed = false;
        for used in 0..2 {
            for r in 0..rows {
                for c in 0..cols {
                    let d = cost[used][r][c];
                    if d == i64::MAX {
                        continue;
                    }
                    let near = [(r.wrapping_sub(1), c), (r + 1, c), (r, c.wrapping_sub(1)), (r, c + 1)];
                    for (nr, nc) in near {
                        if nr >= rows || nc >= cols {
                            continue;
                        }
                        let toll = tolls[nr][nc];
                        if d + toll < cost[used][nr][nc] {
                            cost[used][nr][nc] = d + toll;
                            changed = true;
                        }
                        if used == 0 && d + toll / 2 < cost[1][nr][nc] {
                            cost[1][nr][nc] = d + toll / 2;
                            changed = true;
                        }
                    }
                }
            }
        }
    }
    cost[0][rows - 1][cols - 1].min(cost[1][rows - 1][cols - 1])
}
