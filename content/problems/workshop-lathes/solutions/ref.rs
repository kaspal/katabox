use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn finish_time(jobs: Vec<i64>, lathes: i64) -> i64 {
    let m = (lathes as usize).min(jobs.len());
    let mut free: BinaryHeap<Reverse<i64>> = std::iter::repeat_n(Reverse(0), m).collect();
    let mut latest = 0;
    for job in jobs {
        let Reverse(t) = free.pop().unwrap();
        let end = t + job;
        free.push(Reverse(end));
        latest = latest.max(end);
    }
    latest
}
