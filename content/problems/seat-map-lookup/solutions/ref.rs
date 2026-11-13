pub fn has_ticket(seats: Vec<Vec<i64>>, ticket: i64) -> bool {
    // `c` is one past the current column, so it never goes below zero.
    let (mut r, mut c) = (0, seats[0].len());
    while r < seats.len() && c > 0 {
        let v = seats[r][c - 1];
        if v == ticket {
            return true;
        }
        if v > ticket {
            c -= 1;
        } else {
            r += 1;
        }
    }
    false
}
