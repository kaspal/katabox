pub fn trapped_rain(columns: Vec<i64>) -> i64 {
    // Correct but O(n²): scan both sides of every column for the tallest.
    let mut water = 0;
    for k in 0..columns.len() {
        let left = columns[..=k].iter().copied().max().unwrap();
        let right = columns[k..].iter().copied().max().unwrap();
        water += left.min(right) - columns[k];
    }
    water
}
