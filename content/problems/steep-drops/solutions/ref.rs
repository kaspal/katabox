pub fn steep_drops(prices: Vec<i64>) -> i64 {
    fn sort(a: &mut [i64], buf: &mut Vec<i64>) -> i64 {
        let n = a.len();
        if n < 2 {
            return 0;
        }
        let mid = n / 2;
        let mut count = sort(&mut a[..mid], buf) + sort(&mut a[mid..], buf);
        let (left, right) = a.split_at(mid);
        let mut j = 0;
        for &x in left {
            while j < right.len() && x > 2 * right[j] {
                j += 1;
            }
            count += j as i64;
        }
        buf.clear();
        let (mut i, mut k) = (0, 0);
        while i < left.len() && k < right.len() {
            if left[i] <= right[k] {
                buf.push(left[i]);
                i += 1;
            } else {
                buf.push(right[k]);
                k += 1;
            }
        }
        buf.extend_from_slice(&left[i..]);
        buf.extend_from_slice(&right[k..]);
        a.copy_from_slice(buf);
        count
    }
    let mut a = prices;
    let mut buf = Vec::with_capacity(a.len());
    sort(&mut a, &mut buf)
}
