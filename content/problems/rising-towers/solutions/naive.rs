pub fn longest_rise(heights: Vec<i64>) -> i64 {
    // Correct but O(n²): best[i] from every earlier tower.
    let n = heights.len();
    let mut best = vec![1i64; n];
    for i in 0..n {
        for j in 0..i {
            if heights[j] < heights[i] && best[j] + 1 > best[i] {
                best[i] = best[j] + 1;
            }
        }
    }
    best.into_iter().max().unwrap_or(0)
}
