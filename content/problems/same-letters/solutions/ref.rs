pub fn same_letters(a: String, b: String) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut counts = [0i64; 26];
    for (x, y) in a.bytes().zip(b.bytes()) {
        counts[(x - b'a') as usize] += 1;
        counts[(y - b'a') as usize] -= 1;
    }
    counts.iter().all(|&c| c == 0)
}
