pub fn power_mod(base: i64, exponent: i64) -> i64 {
    // Correct but O(exponent): multiply one factor at a time.
    let b = base % 1_000_003;
    let mut result = 1;
    for _ in 0..exponent {
        result = result * b % 1_000_003;
    }
    result
}
