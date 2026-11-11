pub fn compress(text: String) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(bytes.len());
    let mut start = 0;
    for i in 1..=bytes.len() {
        if i == bytes.len() || bytes[i] != bytes[start] {
            out.push(bytes[start] as char);
            if i - start > 1 {
                out.push_str(&(i - start).to_string());
            }
            start = i;
        }
    }
    out
}
