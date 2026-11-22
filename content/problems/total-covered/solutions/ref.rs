pub fn total_covered(shifts: Vec<Vec<i64>>) -> i64 {
    let mut shifts = shifts;
    shifts.sort();
    let mut total = 0;
    let (mut cur_s, mut cur_e) = (shifts[0][0], shifts[0][1]);
    for sh in &shifts[1..] {
        if sh[0] <= cur_e {
            cur_e = cur_e.max(sh[1]);
        } else {
            total += cur_e - cur_s;
            cur_s = sh[0];
            cur_e = sh[1];
        }
    }
    total + cur_e - cur_s
}
