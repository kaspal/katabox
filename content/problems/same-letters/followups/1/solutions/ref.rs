pub fn twins_after_changes(a: String, b: String, positions: Vec<i64>, letters: String) -> Vec<bool> {
    let mut diff = [0i64; 26];
    for c in a.bytes() {
        diff[(c - b'a') as usize] += 1;
    }
    for c in b.bytes() {
        diff[(c - b'a') as usize] -= 1;
    }
    let mut off = diff.iter().filter(|&&d| d != 0).count() as i64;
    let mut tiles: Vec<u8> = b.into_bytes();
    let mut out = Vec::with_capacity(positions.len());
    for (p, c) in positions.iter().zip(letters.bytes()) {
        let p = *p as usize;
        for (letter, step) in [(tiles[p], 1i64), (c, -1)] {
            let k = (letter - b'a') as usize;
            let before = i64::from(diff[k] != 0);
            diff[k] += step;
            off += i64::from(diff[k] != 0) - before;
        }
        tiles[p] = c;
        out.push(off == 0);
    }
    out
}
