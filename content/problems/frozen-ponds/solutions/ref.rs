fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

pub fn pond_sizes(ice: Vec<String>) -> Vec<i64> {
    let grid: Vec<&[u8]> = ice.iter().map(|r| r.as_bytes()).collect();
    let (h, w) = (grid.len(), grid[0].len());
    let mut parent: Vec<usize> = (0..h * w).collect();
    let mut size = vec![1i64; h * w];
    let mut union = |parent: &mut Vec<usize>, a: usize, b: usize| {
        let (mut a, mut b) = (find(parent, a), find(parent, b));
        if a != b {
            if size[a] < size[b] {
                std::mem::swap(&mut a, &mut b);
            }
            parent[b] = a;
            size[a] += size[b];
        }
    };
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] != b'#' {
                continue;
            }
            if c + 1 < w && grid[r][c + 1] == b'#' {
                union(&mut parent, r * w + c, r * w + c + 1);
            }
            if r + 1 < h && grid[r + 1][c] == b'#' {
                union(&mut parent, r * w + c, (r + 1) * w + c);
            }
        }
    }
    let mut sizes = Vec::new();
    for r in 0..h {
        for c in 0..w {
            let i = r * w + c;
            if grid[r][c] == b'#' && find(&mut parent, i) == i {
                sizes.push(size[i]);
            }
        }
    }
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes
}
