pub fn settle(paid: Vec<f64>) -> Vec<f64> {
    let share = paid.iter().sum::<f64>() / paid.len() as f64;
    paid.iter().map(|p| p - share).collect()
}
