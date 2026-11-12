pub fn prefix_counts(titles: Vec<String>, typed: Vec<String>) -> Vec<i64> {
    // Node k's children are next[k][..]; 0 means "no child" (the root is 0).
    let mut next: Vec<[u32; 26]> = vec![[0; 26]];
    let mut passes: Vec<i64> = vec![0];
    for t in &titles {
        let mut node = 0usize;
        for b in t.bytes() {
            let c = (b - b'a') as usize;
            if next[node][c] == 0 {
                next[node][c] = next.len() as u32;
                next.push([0; 26]);
                passes.push(0);
            }
            node = next[node][c] as usize;
            passes[node] += 1;
        }
    }
    typed
        .iter()
        .map(|p| {
            let mut node = 0usize;
            for b in p.bytes() {
                node = next[node][(b - b'a') as usize] as usize;
                if node == 0 {
                    return 0;
                }
            }
            passes[node]
        })
        .collect()
}
