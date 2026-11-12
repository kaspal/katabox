pub fn prime_counts(limits: Vec<i64>) -> Vec<i64> {
    // Correct but slow: for every question, trial division for every number up to L.
    fn is_prime(x: i64) -> bool {
        if x < 2 {
            return false;
        }
        let mut d = 2;
        while d * d <= x {
            if x % d == 0 {
                return false;
            }
            d += 1;
        }
        true
    }
    limits
        .iter()
        .map(|&l| (2..=l).filter(|&x| is_prime(x)).count() as i64)
        .collect()
}
