use std::collections::HashSet;

pub fn rotation_groups(codes: Vec<String>) -> i64 {
    let mut seen: HashSet<String> = HashSet::new();
    for code in &codes {
        let best = (0..code.len())
            .map(|i| format!("{}{}", &code[i..], &code[..i]))
            .min()
            .unwrap();
        seen.insert(best);
    }
    seen.len() as i64
}
