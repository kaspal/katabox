// Correct but slow: free heights row by row, then every left edge and width, O(R·C²).
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let r: usize = tokens.next().unwrap().parse().unwrap();
    let c: usize = tokens.next().unwrap().parse().unwrap();
    let mut height = vec![0u64; c];
    let mut best = 0;
    for _ in 0..r {
        let row = tokens.next().unwrap().as_bytes();
        for (h, &ch) in height.iter_mut().zip(row) {
            *h = if ch == b'.' { *h + 1 } else { 0 };
        }
        for left in 0..c {
            let mut low = height[left];
            let mut right = left;
            while right < c && low > 0 {
                low = low.min(height[right]);
                best = best.max(low * (right - left + 1) as u64);
                right += 1;
            }
        }
    }
    println!("{best}");
}
