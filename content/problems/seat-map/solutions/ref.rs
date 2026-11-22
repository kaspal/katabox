pub fn seat_map(rows: i64, cols: i64, bookings: Vec<Vec<i64>>) -> Vec<String> {
    let mut hall = vec![vec![b'.'; cols as usize]; rows as usize];
    for b in &bookings {
        hall[b[0] as usize][b[1] as usize] = b'X';
    }
    hall.into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .collect()
}
