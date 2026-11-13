struct Tree {
    mn: Vec<i64>,
    lazy: Vec<i64>,
}

const INF: i64 = i64::MAX / 4;

impl Tree {
    fn push(&mut self, v: usize) {
        let d = self.lazy[v];
        if d != 0 {
            for c in [2 * v, 2 * v + 1] {
                self.mn[c] += d;
                self.lazy[c] += d;
            }
            self.lazy[v] = 0;
        }
    }

    fn shift(&mut self, v: usize, lo: usize, hi: usize, a: usize, b: usize, d: i64) {
        if b < lo || hi < a {
            return;
        }
        if a <= lo && hi <= b {
            self.mn[v] += d;
            self.lazy[v] += d;
            return;
        }
        self.push(v);
        let mid = (lo + hi) / 2;
        self.shift(2 * v, lo, mid, a, b, d);
        self.shift(2 * v + 1, mid + 1, hi, a, b, d);
        self.mn[v] = self.mn[2 * v].min(self.mn[2 * v + 1]);
    }

    fn coldest(&mut self, v: usize, lo: usize, hi: usize, a: usize, b: usize) -> i64 {
        if b < lo || hi < a {
            return INF;
        }
        if a <= lo && hi <= b {
            return self.mn[v];
        }
        self.push(v);
        let mid = (lo + hi) / 2;
        self.coldest(2 * v, lo, mid, a, b).min(self.coldest(2 * v + 1, mid + 1, hi, a, b))
    }
}

pub fn cold_snaps(temps: Vec<i64>, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    let n = temps.len();
    let size = n.next_power_of_two();
    let mut t = Tree { mn: vec![INF; 2 * size], lazy: vec![0; 2 * size] };
    t.mn[size..size + n].copy_from_slice(&temps);
    for v in (1..size).rev() {
        t.mn[v] = t.mn[2 * v].min(t.mn[2 * v + 1]);
    }
    let mut out = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        let (x, y) = (a[0] as usize, a[1] as usize);
        if op == "shift" {
            t.shift(1, 0, size - 1, x, y, a[2]);
        } else {
            out.push(t.coldest(1, 0, size - 1, x, y));
        }
    }
    out
}
