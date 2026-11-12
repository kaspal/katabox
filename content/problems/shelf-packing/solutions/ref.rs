use std::collections::HashSet;

pub fn fewest_shelves(widths: Vec<i64>, width: i64) -> i64 {
    let mut books = widths;
    books.sort_unstable_by(|a, b| b.cmp(a));
    let lower = (books.iter().sum::<i64>() + width - 1) / width;
    let mut ffd: Vec<i64> = Vec::new();
    for &b in &books {
        match ffd.iter().position(|&load| load + b <= width) {
            Some(k) => ffd[k] += b,
            None => ffd.push(b),
        }
    }
    struct Search {
        books: Vec<i64>,
        width: i64,
        lower: i64,
        best: i64,
        loads: Vec<i64>,
    }
    impl Search {
        fn place(&mut self, i: usize) {
            if self.best == self.lower {
                return;
            }
            if i == self.books.len() {
                self.best = self.best.min(self.loads.len() as i64);
                return;
            }
            let b = self.books[i];
            let mut tried = HashSet::new();
            for k in 0..self.loads.len() {
                if self.loads[k] + b <= self.width && tried.insert(self.loads[k]) {
                    self.loads[k] += b;
                    self.place(i + 1);
                    self.loads[k] -= b;
                }
            }
            if (self.loads.len() as i64) + 1 < self.best {
                self.loads.push(b);
                self.place(i + 1);
                self.loads.pop();
            }
        }
    }
    let mut s = Search { books, width, lower, best: ffd.len() as i64, loads: Vec::new() };
    s.place(0);
    s.best
}
