pub fn fence_area(trees: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n·h): gift wrapping, scanning every tree for each corner.
    let mut pts: Vec<(i64, i64)> = trees.iter().map(|t| (t[0], t[1])).collect();
    pts.sort();
    pts.dedup();
    if pts.len() < 3 {
        return 0;
    }
    let cross = |o: (i64, i64), a: (i64, i64), b: (i64, i64)| (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0);
    let dist2 = |a: (i64, i64), b: (i64, i64)| (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2);
    let start = pts[0];
    let mut hull = Vec::new();
    let mut current = start;
    loop {
        hull.push(current);
        let mut candidate = if pts[0] != current { pts[0] } else { pts[1] };
        for &p in &pts {
            if p == current {
                continue;
            }
            let c = cross(current, candidate, p);
            if c < 0 || (c == 0 && dist2(current, p) > dist2(current, candidate)) {
                candidate = p;
            }
        }
        current = candidate;
        if current == start {
            break;
        }
    }
    if hull.len() < 3 {
        return 0;
    }
    let k = hull.len();
    (0..k).map(|i| hull[i].0 * hull[(i + 1) % k].1 - hull[(i + 1) % k].0 * hull[i].1).sum::<i64>().abs()
}
