pub fn busiest_cells(rows: i64, cols: i64, stamps: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(stamps · rows · cols): ink every cell of every stamp.
    let mut ink = vec![vec![0i64; cols as usize]; rows as usize];
    for s in &stamps {
        for row in &mut ink[s[0] as usize..=s[2] as usize] {
            for v in &mut row[s[1] as usize..=s[3] as usize] {
                *v += 1;
            }
        }
    }
    let most = ink.iter().flatten().copied().max().unwrap();
    let cells = ink.iter().flatten().filter(|&&v| v == most).count() as i64;
    vec![most, cells]
}
