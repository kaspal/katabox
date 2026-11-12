pub fn short_names(names: Vec<String>) -> Vec<String> {
    // Correct but O(n²·L): the label is one letter longer than the longest common
    // beginning with any other name (or the whole name).
    let bytes: Vec<&[u8]> = names.iter().map(|s| s.as_bytes()).collect();
    (0..names.len())
        .map(|i| {
            let mut longest = 0;
            for j in 0..names.len() {
                if i != j {
                    let k = bytes[i].iter().zip(bytes[j]).take_while(|(a, b)| a == b).count();
                    longest = longest.max(k);
                }
            }
            names[i][..(longest + 1).min(names[i].len())].to_string()
        })
        .collect()
}
