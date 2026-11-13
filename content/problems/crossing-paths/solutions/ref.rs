fn orient(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> i64 {
    ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)).signum()
}

fn within(a: (i64, i64), b: (i64, i64), p: (i64, i64)) -> bool {
    a.0.min(b.0) <= p.0 && p.0 <= a.0.max(b.0) && a.1.min(b.1) <= p.1 && p.1 <= a.1.max(b.1)
}

pub fn paths_cross(pairs: Vec<Vec<i64>>) -> Vec<bool> {
    pairs
        .iter()
        .map(|q| {
            let (p1, p2, p3, p4) = ((q[0], q[1]), (q[2], q[3]), (q[4], q[5]), (q[6], q[7]));
            let (d1, d2) = (orient(p1, p2, p3), orient(p1, p2, p4));
            let (d3, d4) = (orient(p3, p4, p1), orient(p3, p4, p2));
            (d1 * d2 < 0 && d3 * d4 < 0)
                || (d1 == 0 && within(p1, p2, p3))
                || (d2 == 0 && within(p1, p2, p4))
                || (d3 == 0 && within(p3, p4, p1))
                || (d4 == 0 && within(p3, p4, p2))
        })
        .collect()
}
