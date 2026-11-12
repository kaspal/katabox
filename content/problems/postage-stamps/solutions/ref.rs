pub fn exact_postage(values: Vec<i64>, target: i64) -> Vec<i64> {
    struct Search<'a> {
        values: &'a [i64],
        suffix: Vec<i64>,
        chosen: Vec<i64>,
        best: Option<Vec<i64>>,
    }
    impl Search<'_> {
        fn go(&mut self, i: usize, missing: i64) {
            if missing == 0 {
                if self.best.as_ref().is_none_or(|b| self.chosen.len() < b.len()) {
                    self.best = Some(self.chosen.clone());
                }
                return;
            }
            if i == self.values.len() || missing > self.suffix[i] {
                return;
            }
            if self.best.as_ref().is_some_and(|b| self.chosen.len() + 1 >= b.len()) {
                return;
            }
            if self.values[i] <= missing {
                self.chosen.push(i as i64);
                self.go(i + 1, missing - self.values[i]);
                self.chosen.pop();
            }
            self.go(i + 1, missing);
        }
    }
    let mut suffix = vec![0i64; values.len() + 1];
    for i in (0..values.len()).rev() {
        suffix[i] = suffix[i + 1] + values[i];
    }
    let mut s = Search { values: &values, suffix, chosen: Vec::new(), best: None };
    s.go(0, target);
    s.best.unwrap_or_default()
}
