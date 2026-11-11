pub fn min_speed(pages: Vec<i64>, hours: i64) -> i64 {
    // Correct but O(n·answer): try speeds 1, 2, 3, ... one by one.
    let mut speed = 1;
    loop {
        let need: i64 = pages.iter().map(|p| (p + speed - 1) / speed).sum();
        if need <= hours {
            return speed;
        }
        speed += 1;
    }
}
