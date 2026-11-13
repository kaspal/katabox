use std::collections::BinaryHeap;

pub fn skyline(buildings: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut order = buildings.clone();
    order.sort_by_key(|b| b[0]);
    let mut xs: Vec<i64> = buildings.iter().flat_map(|b| [b[0], b[1]]).collect();
    xs.sort_unstable();
    xs.dedup();
    let mut heap: BinaryHeap<(i64, i64)> = BinaryHeap::new();
    let (mut out, mut i, mut last) = (Vec::new(), 0, 0);
    for x in xs {
        while i < order.len() && order[i][0] <= x {
            heap.push((order[i][2], order[i][1]));
            i += 1;
        }
        while heap.peek().is_some_and(|&(_, right)| right <= x) {
            heap.pop();
        }
        let h = heap.peek().map_or(0, |&(h, _)| h);
        if h != last {
            out.push(vec![x, h]);
            last = h;
        }
    }
    out
}
