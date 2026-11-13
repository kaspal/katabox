pub fn fuel_stops(target: i64, fuel: i64, stations: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n²): far[t] = the farthest reach with t stops, updated per station.
    let mut far = vec![0i64; stations.len() + 1];
    far[0] = fuel;
    for (i, s) in stations.iter().enumerate() {
        for t in (0..=i).rev() {
            if far[t] >= s[0] && far[t] + s[1] > far[t + 1] {
                far[t + 1] = far[t] + s[1];
            }
        }
    }
    far.iter().position(|&r| r >= target).map_or(-1, |t| t as i64)
}
