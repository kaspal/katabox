use std::collections::BinaryHeap;

pub fn nearest_stalls(stalls: Vec<Vec<i64>>, k: i64) -> Vec<i64> {
    let k = k as usize;
    let mut heap: BinaryHeap<(i64, i64)> = BinaryHeap::new();
    for (i, s) in stalls.iter().enumerate() {
        heap.push((s[0] * s[0] + s[1] * s[1], i as i64));
        if heap.len() > k {
            heap.pop();
        }
    }
    heap.into_sorted_vec().into_iter().map(|(_, i)| i).collect()
}
