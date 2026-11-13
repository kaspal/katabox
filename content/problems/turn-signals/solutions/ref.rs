pub fn turn_signals(routes: Vec<Vec<i64>>) -> Vec<String> {
    routes
        .iter()
        .map(|r| {
            let cross = (r[2] - r[0]) * (r[5] - r[3]) - (r[3] - r[1]) * (r[4] - r[2]);
            match cross.signum() {
                1 => "left",
                -1 => "right",
                _ => "straight",
            }
            .to_string()
        })
        .collect()
}
