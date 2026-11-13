pub fn stock_ledger(stock: Vec<i64>, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(n·q): every "total" walks its shelves.
    let mut shelves = stock;
    let mut out = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        if op == "set" {
            shelves[a[0] as usize] = a[1];
        } else {
            out.push(shelves[a[0] as usize..=a[1] as usize].iter().sum());
        }
    }
    out
}
