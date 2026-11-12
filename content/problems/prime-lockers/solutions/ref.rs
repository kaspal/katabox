pub fn prime_counts(limits: Vec<i64>) -> Vec<i64> {
    let m = *limits.iter().max().unwrap() as usize;
    let mut composite = vec![false; m + 1];
    let mut p = 2;
    while p * p <= m {
        if !composite[p] {
            let mut q = p * p;
            while q <= m {
                composite[q] = true;
                q += p;
            }
        }
        p += 1;
    }
    let mut count = vec![0i64; m + 1];
    for x in 2..=m {
        count[x] = count[x - 1] + if composite[x] { 0 } else { 1 };
    }
    limits.iter().map(|&x| count[x as usize]).collect()
}
