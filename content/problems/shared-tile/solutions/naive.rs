pub fn largest_tile(width: i64, height: i64) -> i64 {
    // Correct but O(min(width, height)): try every size from the smaller side down.
    let mut s = width.min(height);
    while width % s != 0 || height % s != 0 {
        s -= 1;
    }
    s
}
