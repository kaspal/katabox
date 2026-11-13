pub fn where_is_point(fence: Vec<Vec<i64>>, points: Vec<Vec<i64>>) -> Vec<String> {
    let n = fence.len();
    points
        .iter()
        .map(|p| {
            let (px, py) = (p[0], p[1]);
            let mut inside = false;
            for i in 0..n {
                let (ax, ay) = (fence[i][0], fence[i][1]);
                let (bx, by) = (fence[(i + 1) % n][0], fence[(i + 1) % n][1]);
                if (bx - ax) * (py - ay) - (by - ay) * (px - ax) == 0
                    && ax.min(bx) <= px
                    && px <= ax.max(bx)
                    && ay.min(by) <= py
                    && py <= ay.max(by)
                {
                    return "boundary".to_string();
                }
                if (ay > py) != (by > py) {
                    let t = (py - ay) * (bx - ax) - (px - ax) * (by - ay);
                    if (t > 0) == (by > ay) {
                        inside = !inside;
                    }
                }
            }
            (if inside { "inside" } else { "outside" }).to_string()
        })
        .collect()
}
