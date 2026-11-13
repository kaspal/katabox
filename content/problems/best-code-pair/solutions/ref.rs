pub fn best_xor_pair(codes: Vec<i64>) -> i64 {
    // Binary trie: children[node][bit], 0 means "no child".
    let mut children: Vec<[u32; 2]> = vec![[0, 0]];
    let mut best = 0;
    for (i, &x) in codes.iter().enumerate() {
        if i > 0 {
            let (mut node, mut value) = (0usize, 0i64);
            for bit in (0..20).rev() {
                let b = ((x >> bit) & 1) as usize;
                if children[node][b ^ 1] != 0 {
                    value |= 1 << bit;
                    node = children[node][b ^ 1] as usize;
                } else {
                    node = children[node][b] as usize;
                }
            }
            best = best.max(value);
        }
        let mut node = 0usize;
        for bit in (0..20).rev() {
            let b = ((x >> bit) & 1) as usize;
            if children[node][b] == 0 {
                children[node][b] = children.len() as u32;
                children.push([0, 0]);
            }
            node = children[node][b] as usize;
        }
    }
    best
}
