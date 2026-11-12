pub fn short_names(names: Vec<String>) -> Vec<String> {
    // Node k's children are next[k][..]; 0 means "no child" (the root is 0).
    let mut next: Vec<[u32; 26]> = vec![[0; 26]];
    let mut passes: Vec<u32> = vec![0];
    for s in &names {
        let mut node = 0usize;
        for b in s.bytes() {
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
    names
        .iter()
        .map(|s| {
            let mut node = 0usize;
            for (i, b) in s.bytes().enumerate() {
                node = next[node][(b - b'a') as usize] as usize;
                if passes[node] == 1 {
                    return s[..=i].to_string();
                }
            }
            s.clone()
        })
        .collect()
}
