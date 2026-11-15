use std::io::{self, Read, Write};

fn root(parent: &mut [i64], mut x: usize) -> usize {
    while parent[x] as usize != x {
        parent[x] = parent[parent[x] as usize];
        x = parent[x] as usize;
    }
    x
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<usize>().unwrap());
    let r = tokens.next().unwrap();
    let c = tokens.next().unwrap();
    let q = tokens.next().unwrap();
    let width = c + 2; // a border of sea around the map: no bounds checks
    let mut parent = vec![-1i64; (r + 2) * width]; // -1: sea; otherwise union-find over land
    let mut islands: i64 = 0;
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..q {
        let cell = tokens.next().unwrap() * width + tokens.next().unwrap();
        if parent[cell] == -1 {
            // A new island, then one fewer for every different island it touches.
            parent[cell] = cell as i64;
            islands += 1;
            for next in [cell + 1, cell - 1, cell + width, cell - width] {
                if parent[next] != -1 {
                    let a = root(&mut parent, cell);
                    let b = root(&mut parent, next);
                    if a != b {
                        parent[b] = a as i64;
                        islands -= 1;
                    }
                }
            }
        }
        writeln!(out, "{islands}").unwrap();
    }
}
