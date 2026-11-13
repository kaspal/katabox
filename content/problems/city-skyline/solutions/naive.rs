pub fn skyline(buildings: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    // Correct but O(n²): at every critical position, scan every building.
    let mut xs: Vec<i64> = buildings.iter().flat_map(|b| [b[0], b[1]]).collect();
    xs.sort_unstable();
    xs.dedup();
    let (mut out, mut last) = (Vec::new(), 0);
    for x in xs {
        let mut h = 0;
        for b in &buildings {
            if b[0] <= x && x < b[1] && b[2] > h {
                h = b[2];
            }
        }
        if h != last {
            out.push(vec![x, h]);
            last = h;
        }
    }
    out
}
