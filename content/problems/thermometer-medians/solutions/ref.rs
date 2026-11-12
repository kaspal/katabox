use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn running_medians(readings: Vec<i64>) -> Vec<f64> {
    let mut low: BinaryHeap<i64> = BinaryHeap::new();
    let mut high: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut out = Vec::with_capacity(readings.len());
    for x in readings {
        match low.peek() {
            Some(&top) if x > top => high.push(Reverse(x)),
            _ => low.push(x),
        }
        if low.len() > high.len() + 1 {
            let v = low.pop().unwrap();
            high.push(Reverse(v));
        } else if high.len() > low.len() {
            let Reverse(v) = high.pop().unwrap();
            low.push(v);
        }
        let lo = *low.peek().unwrap();
        out.push(if low.len() > high.len() {
            lo as f64
        } else {
            (lo + high.peek().unwrap().0) as f64 / 2.0
        });
    }
    out
}
