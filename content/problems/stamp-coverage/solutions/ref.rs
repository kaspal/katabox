pub fn busiest_cells(rows: i64, cols: i64, stamps: Vec<Vec<i64>>) -> Vec<i64> {
    let (rows, cols) = (rows as usize, cols as usize);
    let w = cols + 1;
    let mut d = vec![0i64; (rows + 1) * w];
    for s in &stamps {
        let (t, l, b, r) = (s[0] as usize, s[1] as usize, s[2] as usize + 1, s[3] as usize + 1);
        d[t * w + l] += 1;
        d[t * w + r] -= 1;
        d[b * w + l] -= 1;
        d[b * w + r] += 1;
    }
    let (mut most, mut cells) = (0i64, 0i64);
    let mut above = vec![0i64; cols];
    for i in 0..rows {
        let mut run = 0;
        for j in 0..cols {
            run += d[i * w + j];
            above[j] += run;
            let v = above[j];
            if v > most {
                most = v;
                cells = 1;
            } else if v == most {
                cells += 1;
            }
        }
    }
    vec![most, cells]
}
