struct Search<'a> {
    grid: Vec<&'a [u8]>,
    next: Vec<[u32; 26]>,
    word: Vec<Option<usize>>,
    used: Vec<Vec<bool>>,
    found: Vec<bool>,
}

impl Search<'_> {
    fn dfs(&mut self, r: usize, c: usize, node: usize) {
        let child = self.next[node][(self.grid[r][c] - b'a') as usize] as usize;
        if child == 0 {
            return;
        }
        if let Some(w) = self.word[child] {
            self.found[w] = true;
        }
        self.used[r][c] = true;
        let (rows, cols) = (self.grid.len(), self.grid[0].len());
        if r + 1 < rows && !self.used[r + 1][c] {
            self.dfs(r + 1, c, child);
        }
        if r > 0 && !self.used[r - 1][c] {
            self.dfs(r - 1, c, child);
        }
        if c + 1 < cols && !self.used[r][c + 1] {
            self.dfs(r, c + 1, child);
        }
        if c > 0 && !self.used[r][c - 1] {
            self.dfs(r, c - 1, child);
        }
        self.used[r][c] = false;
    }
}

pub fn find_words(grid: Vec<String>, words: Vec<String>) -> Vec<String> {
    let mut next: Vec<[u32; 26]> = vec![[0; 26]];
    let mut word: Vec<Option<usize>> = vec![None];
    for (i, w) in words.iter().enumerate() {
        let mut node = 0usize;
        for b in w.bytes() {
            let k = (b - b'a') as usize;
            if next[node][k] == 0 {
                next[node][k] = next.len() as u32;
                next.push([0; 26]);
                word.push(None);
            }
            node = next[node][k] as usize;
        }
        word[node] = Some(i);
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut s = Search {
        grid: grid.iter().map(|r| r.as_bytes()).collect(),
        next,
        word,
        used: vec![vec![false; cols]; rows],
        found: vec![false; words.len()],
    };
    for r in 0..rows {
        for c in 0..cols {
            s.dfs(r, c, 0);
        }
    }
    let mut out: Vec<String> = words
        .iter()
        .enumerate()
        .filter(|&(i, _)| s.found[i])
        .map(|(_, w)| w.clone())
        .collect();
    out.sort();
    out.dedup();
    out
}
