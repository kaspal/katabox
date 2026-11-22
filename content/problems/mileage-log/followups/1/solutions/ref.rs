pub fn long_days(miles: Vec<i64>, trips: Vec<Vec<i64>>) -> Vec<i64> {
    let n = miles.len();
    let mut tree = vec![0i64; n + 1];
    let mut days: Vec<usize> = (0..n).collect();
    days.sort_by(|&x, &y| miles[y].cmp(&miles[x]));
    let mut order: Vec<usize> = (0..trips.len()).collect();
    order.sort_by(|&x, &y| trips[y][2].cmp(&trips[x][2]));
    let prefix = |tree: &Vec<i64>, mut i: usize| {
        let mut s = 0;
        while i > 0 {
            s += tree[i];
            i &= i - 1;
        }
        s
    };
    let mut out = vec![0i64; trips.len()];
    let mut p = 0;
    for j in order {
        let (a, b, x) = (trips[j][0] as usize, trips[j][1] as usize, trips[j][2]);
        while p < n && miles[days[p]] >= x {
            let mut i = days[p] + 1;
            while i <= n {
                tree[i] += 1;
                i += i & i.wrapping_neg();
            }
            p += 1;
        }
        out[j] = prefix(&tree, b + 1) - prefix(&tree, a);
    }
    out
}
