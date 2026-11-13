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

pub fn stock_ledger(stock: Vec<i64>, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    let mut tree = vec![0i64; stock.len() + 1];
    for (i, &v) in stock.iter().enumerate() {
        fenwick_add(&mut tree, i, v);
    }
    let mut current = stock;
    let mut out = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        let (x, y) = (a[0] as usize, a[1]);
        if op == "set" {
            fenwick_add(&mut tree, x, y - current[x]);
            current[x] = y;
        } else {
            out.push(fenwick_prefix(&tree, y as usize + 1) - fenwick_prefix(&tree, x));
        }
    }
    out
}
