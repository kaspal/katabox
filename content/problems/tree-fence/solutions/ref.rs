fn cross(o: (i64, i64), a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn half(points: impl Iterator<Item = (i64, i64)>) -> Vec<(i64, i64)> {
    let mut hull: Vec<(i64, i64)> = Vec::new();
    for p in points {
        while hull.len() >= 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0 {
            hull.pop();
        }
        hull.push(p);
    }
    hull
}

pub fn fence_area(trees: Vec<Vec<i64>>) -> i64 {
    let mut pts: Vec<(i64, i64)> = trees.iter().map(|t| (t[0], t[1])).collect();
    pts.sort();
    let mut lower = half(pts.iter().copied());
    let mut upper = half(pts.iter().rev().copied());
    lower.pop();
    upper.pop();
    lower.extend(upper);
    let hull = lower;
    if hull.len() < 3 {
        return 0;
    }
    (0..hull.len())
        .map(|i| {
            let (a, b) = (hull[i], hull[(i + 1) % hull.len()]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum()
}
