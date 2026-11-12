pub fn largest_tile(width: i64, height: i64) -> i64 {
    let (mut a, mut b) = (width, height);
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
