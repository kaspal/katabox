// Correct but slow: test every lattice point of the bounding box, O(area · n).
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let n = tokens.next().unwrap() as usize;
    let pts: Vec<(i64, i64)> = (0..n)
        .map(|_| (tokens.next().unwrap(), tokens.next().unwrap()))
        .collect();
    let edges: Vec<((i64, i64), (i64, i64))> = (0..n).map(|i| (pts[i], pts[(i + 1) % n])).collect();
    let (x0, x1) = (pts.iter().map(|p| p.0).min().unwrap(), pts.iter().map(|p| p.0).max().unwrap());
    let (y0, y1) = (pts.iter().map(|p| p.1).min().unwrap(), pts.iter().map(|p| p.1).max().unwrap());
    let mut count: u64 = 0;
    for px in x0..=x1 {
        for py in y0..=y1 {
            let mut crossings = 0;
            let mut boundary = false;
            for &((ax, ay), (bx, by)) in &edges {
                let t = (px - ax) * (by - ay) - (bx - ax) * (py - ay);
                if t == 0 && ax.min(bx) <= px && px <= ax.max(bx) && ay.min(by) <= py && py <= ay.max(by) {
                    boundary = true;
                    break;
                }
                if (ay > py) != (by > py) && (t < 0) == (by > ay) {
                    crossings += 1;
                }
            }
            if !boundary && crossings % 2 == 1 {
                count += 1;
            }
        }
    }
    let twice: i64 = edges.iter().map(|&((ax, ay), (bx, by))| ax * by - bx * ay).sum::<i64>().abs();
    println!("{}.{} {count}", twice / 2, if twice % 2 == 1 { 5 } else { 0 });
}
