fn fenwick_add(tree: &mut [i64], i: usize, delta: i64) {
    let mut k = i + 1;
    while k < tree.len() {
        tree[k] += delta;
        k += k & k.wrapping_neg();
    }
}

fn fenwick_prefix(tree: &[i64], i: usize) -> i64 {
    let (mut k, mut total) = (i, 0);
    while k > 0 {
        total += tree[k];
        k -= k & k.wrapping_neg();
    }
    total
}

pub fn gauge_readings(readings: Vec<i64>, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    let n = readings.len();
    let mut tree = vec![0i64; n + 1];
    let mut out = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        if op == "add" {
            fenwick_add(&mut tree, a[0] as usize, a[2]);
            if (a[1] as usize) + 1 < n {
                fenwick_add(&mut tree, a[1] as usize + 1, -a[2]);
            }
        } else {
            let i = a[0] as usize;
            out.push(readings[i] + fenwick_prefix(&tree, i + 1));
        }
    }
    out
}
