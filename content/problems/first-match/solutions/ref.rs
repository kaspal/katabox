pub fn first_matches(words: Vec<String>, prefixes: Vec<String>) -> Vec<String> {
    let mut sorted = words;
    sorted.sort_unstable();
    sorted.dedup();
    // Node k's children are next[k][..]; 0 means "no child" (the root is 0).
    let mut next: Vec<[u32; 26]> = vec![[0; 26]];
    let mut best: Vec<usize> = vec![usize::MAX];
    for (wi, w) in sorted.iter().enumerate() {
        let mut node = 0usize;
        for b in w.bytes() {
            let c = (b - b'a') as usize;
            if next[node][c] == 0 {
                next[node][c] = next.len() as u32;
                next.push([0; 26]);
                best.push(wi);
            }
            node = next[node][c] as usize;
        }
    }
    prefixes
        .iter()
        .map(|p| {
            let mut node = 0usize;
            for b in p.bytes() {
                node = next[node][(b - b'a') as usize] as usize;
                if node == 0 {
                    return String::new();
                }
            }
            sorted[best[node]].clone()
        })
        .collect()
}
