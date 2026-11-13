const DIGITS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn convert_base(number: String, source: i64, target: i64) -> String {
    let mut value: i64 = 0;
    for b in number.bytes() {
        let d = DIGITS.iter().position(|&x| x == b).unwrap() as i64;
        value = value * source + d;
    }
    if value == 0 {
        return "0".to_string();
    }
    let mut out = Vec::new();
    while value > 0 {
        out.push(DIGITS[(value % target) as usize]);
        value /= target;
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}
