pub fn trapped_rain(columns: Vec<i64>) -> i64 {
    // `j` is one past the right pointer, so it never goes below zero.
    let (mut i, mut j) = (0, columns.len());
    let (mut left_max, mut right_max, mut water) = (0, 0, 0);
    while i < j {
        left_max = left_max.max(columns[i]);
        right_max = right_max.max(columns[j - 1]);
        if left_max <= right_max {
            water += left_max - columns[i];
            i += 1;
        } else {
            water += right_max - columns[j - 1];
            j -= 1;
        }
    }
    water
}
