use std::io::{self, Read};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let pts: Vec<(i64, i64)> = (0..n)
        .map(|_| (tokens.next().unwrap(), tokens.next().unwrap()))
        .collect();
    let mut twice = 0; // twice the area: the shoelace formula, in whole numbers
    let mut boundary = 0; // lattice points on the edges: an edge (dx, dy) holds gcd(|dx|, |dy|)
    for i in 0..n {
        let (ax, ay) = pts[i];
        let (bx, by) = pts[(i + 1) % n];
        twice += ax * by - bx * ay;
        boundary += gcd((bx - ax).abs(), (by - ay).abs());
    }
    let twice = twice.abs();
    // Pick's theorem: area = inside + boundary / 2 - 1, so inside = (2·area - boundary + 2) / 2.
    let inside = (twice - boundary + 2) / 2;
    println!("{}.{} {inside}", twice / 2, if twice % 2 == 1 { 5 } else { 0 });
}
