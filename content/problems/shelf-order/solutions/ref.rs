#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Chunk {
    // Digits first (declared first), compared by length without leading zeros, then text.
    Digits(usize, String),
    Letters(String),
}

fn chunk_key(label: &str) -> Vec<Chunk> {
    let b = label.as_bytes();
    let mut key = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let digit = b[i].is_ascii_digit();
        let mut j = i;
        while j < b.len() && b[j].is_ascii_digit() == digit {
            j += 1;
        }
        let chunk = &label[i..j];
        if digit {
            let digits = chunk.trim_start_matches('0').to_string();
            key.push(Chunk::Digits(digits.len(), digits));
        } else {
            key.push(Chunk::Letters(chunk.to_string()));
        }
        i = j;
    }
    key
}

pub fn natural_order(labels: Vec<String>) -> Vec<String> {
    let mut keyed: Vec<(Vec<Chunk>, String)> = labels.into_iter().map(|l| (chunk_key(&l), l)).collect();
    keyed.sort_by(|a, b| a.0.cmp(&b.0)); // stable
    keyed.into_iter().map(|(_, l)| l).collect()
}
