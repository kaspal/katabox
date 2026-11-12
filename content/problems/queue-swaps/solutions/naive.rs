pub fn swaps_to_sort(heights: Vec<i64>) -> i64 {
    // Correct but O(n²): count every pair that is out of order.
    let mut count = 0i64;
    for i in 0..heights.len() {
        for j in i + 1..heights.len() {
            if heights[i] > heights[j] {
                count += 1;
            }
        }
    }
    count
}
