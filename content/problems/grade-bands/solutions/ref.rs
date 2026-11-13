pub fn grade_bands(scores: Vec<i64>) -> Vec<String> {
    scores
        .iter()
        .map(|&s| {
            let band = if s >= 90 {
                "A"
            } else if s >= 80 {
                "B"
            } else if s >= 70 {
                "C"
            } else if s >= 60 {
                "D"
            } else {
                "F"
            };
            band.to_string()
        })
        .collect()
}
