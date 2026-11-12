pub fn swaps_to_sort(heights: Vec<i64>) -> i64 {
    // Bottom-up merge sort: merge runs of width 1, 2, 4, … between two buffers.
    let n = heights.len();
    let mut a = heights;
    let mut b = vec![0i64; n];
    let mut count = 0i64;
    let mut width = 1;
    while width < n {
        let mut lo = 0;
        while lo < n {
            let mid = (lo + width).min(n);
            let hi = (lo + 2 * width).min(n);
            let (mut i, mut j, mut k) = (lo, mid, lo);
            while i < mid && j < hi {
                if a[i] <= a[j] {
                    b[k] = a[i];
                    i += 1;
                } else {
                    b[k] = a[j];
                    j += 1;
                    count += (mid - i) as i64;
                }
                k += 1;
            }
            b[k..k + mid - i].copy_from_slice(&a[i..mid]);
            k += mid - i;
            b[k..k + hi - j].copy_from_slice(&a[j..hi]);
            lo += 2 * width;
        }
        std::mem::swap(&mut a, &mut b);
        width *= 2;
    }
    count
}
