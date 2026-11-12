pub fn pond_sizes(ice: Vec<String>) -> Vec<i64> {
    // Correct but slow: every frozen cell copies the smallest label among its frozen
    // neighbours, round after round, until nothing changes. A winding pond needs one
    // round per cell along it: O((r·c)²).
    let grid: Vec<&[u8]> = ice.iter().map(|r| r.as_bytes()).collect();
    let (h, w) = (grid.len() as i64, grid[0].len() as i64);
    let mut label: Vec<i64> = (0..h * w)
        .map(|i| if grid[(i / w) as usize][(i % w) as usize] == b'#' { i } else { -1 })
        .collect();
    let mut changed = true;
    while changed {
        changed = false;
        for r in 0..h {
            for c in 0..w {
                let i = (r * w + c) as usize;
                if label[i] < 0 {
                    continue;
                }
                let mut best = label[i];
                for (y, x) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                    if y >= 0 && y < h && x >= 0 && x < w {
                        let l = label[(y * w + x) as usize];
                        if l >= 0 && l < best {
                            best = l;
                        }
                    }
                }
                if best < label[i] {
                    label[i] = best;
                    changed = true;
                }
            }
        }
    }
    let mut counts = std::collections::HashMap::new();
    for &l in &label {
        if l >= 0 {
            *counts.entry(l).or_insert(0i64) += 1;
        }
    }
    let mut sizes: Vec<i64> = counts.into_values().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes
}
