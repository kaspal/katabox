pub fn two_sum(nums: Vec<i64>, target: i64) -> Vec<i64> {
    // Correct but O(n²): must run out of time on the max-size hidden tests.
    let n = nums.len();
    for i in 0..n {
        for j in i + 1..n {
            if nums[i] + nums[j] == target {
                return vec![i as i64, j as i64];
            }
        }
    }
    Vec::new()
}
