use std::collections::VecDeque;

pub fn smallest_squad(skills: i64, people: Vec<i64>) -> i64 {
    let full = (1usize << skills) - 1;
    let mut moves: Vec<usize> = people.iter().map(|&p| p as usize).collect();
    moves.sort_unstable();
    moves.dedup();
    let mut dist = vec![-1i64; full + 1];
    dist[0] = 0;
    let mut queue = VecDeque::from([0usize]);
    while let Some(mask) = queue.pop_front() {
        if mask == full {
            return dist[mask];
        }
        for &p in &moves {
            let m = mask | p;
            if dist[m] < 0 {
                dist[m] = dist[mask] + 1;
                queue.push_back(m);
            }
        }
    }
    -1
}
