pub fn live_counts(ops: Vec<String>, words: Vec<String>) -> Vec<i64> {
    let mut children: Vec<[u32; 26]> = vec![[0; 26]];
    let mut count: Vec<i64> = vec![0];
    let mut out = Vec::new();
    for (op, w) in ops.iter().zip(words.iter()) {
        if op == "count" {
            let mut node = 0usize;
            let mut found = true;
            for c in w.bytes() {
                let nxt = children[node][(c - b'a') as usize] as usize;
                if nxt == 0 {
                    found = false;
                    break;
                }
                node = nxt;
            }
            out.push(if found { count[node] } else { 0 });
            continue;
        }
        let step = if op == "add" { 1 } else { -1 };
        let mut node = 0usize;
        for c in w.bytes() {
            let k = (c - b'a') as usize;
            if children[node][k] == 0 {
                children.push([0; 26]);
                count.push(0);
                children[node][k] = (count.len() - 1) as u32;
            }
            node = children[node][k] as usize;
            count[node] += step;
        }
    }
    out
}
