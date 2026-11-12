use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn gentlest_climb(heights: Vec<Vec<i64>>) -> i64 {
    let (rows, cols) = (heights.len(), heights[0].len());
    let n = rows * cols;
    let flat: Vec<i64> = heights.into_iter().flatten().collect();
    let mut best = vec![i64::MAX; n];
    best[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i64, 0usize)));
    while let Some(Reverse((d, i))) = heap.pop() {
        if i == n - 1 {
            return d;
        }
        if d > best[i] {
            continue;
        }
        let (r, c) = (i / cols, i % cols);
        let mut visit = |j: usize, heap: &mut BinaryHeap<Reverse<(i64, usize)>>| {
            let nd = d.max((flat[j] - flat[i]).abs());
            if nd < best[j] {
                best[j] = nd;
                heap.push(Reverse((nd, j)));
            }
        };
        if r > 0 {
            visit(i - cols, &mut heap);
        }
        if r + 1 < rows {
            visit(i + cols, &mut heap);
        }
        if c > 0 {
            visit(i - 1, &mut heap);
        }
        if c + 1 < cols {
            visit(i + 1, &mut heap);
        }
    }
    0
}
