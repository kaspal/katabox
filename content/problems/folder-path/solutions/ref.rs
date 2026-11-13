pub fn clean_path(path: String) -> String {
    let mut stack: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            ".." => {
                stack.pop();
            }
            "" | "." => {}
            name => stack.push(name),
        }
    }
    format!("/{}", stack.join("/"))
}
