const P: i64 = 1_000_003;

pub fn power_mod(base: i64, exponent: i64) -> i64 {
    let (mut result, mut square, mut e) = (1, base % P, exponent);
    while e > 0 {
        if e % 2 == 1 {
            result = result * square % P;
        }
        square = square * square % P;
        e /= 2;
    }
    result
}
