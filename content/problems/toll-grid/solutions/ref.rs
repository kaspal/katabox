use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn cheapest_crossing(tolls: Vec<Vec<i64>>) -> i64 {
    let (rows, cols) = (tolls.len(), tolls[0].len());
    let cells = rows * cols;
    // State s = used * cells + r * cols + c.
    let mut cost = vec![i64::MAX; 2 * cells];
    let mut heap = BinaryHeap::new();
    let start = tolls[0][0];
    cost[0] = start;
    cost[cells] = start / 2;
    heap.push(Reverse((start, 0usize)));
    heap.push(Reverse((start / 2, cells)));
    while let Some(Reverse((d, s))) = heap.pop() {
        if d > cost[s] {
            continue;
        }
        let used = s >= cells;
        let cell = if used { s - cells } else { s };
        let (r, c) = (cell / cols, cell % cols);
        let near = [
            (r.wrapping_sub(1), c),
            (r + 1, c),
            (r, c.wrapping_sub(1)),
            (r, c + 1),
        ];
        for (nr, nc) in near {
            if nr >= rows || nc >= cols {
                continue;
            }
            let toll = tolls[nr][nc];
            let t = if used { cells } else { 0 } + nr * cols + nc;
            if d + toll < cost[t] {
                cost[t] = d + toll;
                heap.push(Reverse((d + toll, t)));
            }
            if !used {
                let u = cells + nr * cols + nc;
                if d + toll / 2 < cost[u] {
                    cost[u] = d + toll / 2;
                    heap.push(Reverse((d + toll / 2, u)));
                }
            }
        }
    }
    cost[cells - 1].min(cost[2 * cells - 1])
}
