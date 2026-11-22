fn add(t: &mut [i64], mut i: usize, v: i64) {
    while i < t.len() {
        t[i] += v;
        i += i & i.wrapping_neg();
    }
}

fn query(t: &[i64], mut i: usize) -> i64 {
    let mut s = 0;
    while i > 0 {
        s += t[i];
        i -= i & i.wrapping_neg();
    }
    s
}

pub fn row_ledger(stock: Vec<i64>, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    let n = stock.len();
    let mut b1 = vec![0i64; n + 2];
    let mut b2 = vec![0i64; n + 2];
    let mut range_add = |b1: &mut Vec<i64>, b2: &mut Vec<i64>, a: usize, b: usize, v: i64| {
        add(b1, a, v);
        add(b1, b + 1, -v);
        add(b2, a, v * (a as i64 - 1));
        add(b2, b + 1, -v * b as i64);
    };
    for (i, &s) in stock.iter().enumerate() {
        range_add(&mut b1, &mut b2, i + 1, i + 1, s);
    }
    let prefix = |b1: &[i64], b2: &[i64], i: usize| query(b1, i) * i as i64 - query(b2, i);
    let mut out = Vec::new();
    for (op, a) in ops.iter().zip(args.iter()) {
        if op == "add" {
            range_add(&mut b1, &mut b2, a[0] as usize + 1, a[1] as usize + 1, a[2]);
        } else {
            out.push(prefix(&b1, &b2, a[1] as usize + 1) - prefix(&b1, &b2, a[0] as usize));
        }
    }
    out
}
