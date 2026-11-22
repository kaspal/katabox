pub fn closest_sum(nums: Vec<i64>, target: i64) -> i64 {
    let mut a = nums;
    a.sort_unstable();
    let (mut i, mut j) = (0usize, a.len() - 1);
    let mut best = a[0] + a[1];
    while i < j {
        let s = a[i] + a[j];
        let (d, bd) = ((s - target).abs(), (best - target).abs());
        if d < bd || (d == bd && s < best) {
            best = s;
        }
        if s < target {
            i += 1;
        } else if s > target {
            j -= 1;
        } else {
            return s;
        }
    }
    best
}
