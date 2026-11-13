pub fn road_repairs(length: i64, repairs: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(length·repairs): add every repair section by section.
    let mut road = vec![0i64; length as usize];
    for r in &repairs {
        for v in &mut road[r[0] as usize..=r[1] as usize] {
            *v += r[2];
        }
    }
    road
}
