pub fn average_grade(grades: Vec<i64>) -> f64 {
    (grades.iter().sum::<i64>() / grades.len() as i64) as f64
}
