pub fn cheapest_k(prices: Vec<i64>, k: i64) -> i64 {
    let mut heap = std::collections::BinaryHeap::new();
    for p in prices {
        heap.push(p);
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.iter().sum()
}
