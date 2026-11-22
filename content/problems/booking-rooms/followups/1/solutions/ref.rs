use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn assign_rooms(meetings: Vec<Vec<i64>>) -> Vec<i64> {
    let mut order: Vec<usize> = (0..meetings.len()).collect();
    order.sort_by_key(|&i| (meetings[i][0], i));
    let mut free: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut busy: BinaryHeap<Reverse<(i64, i64)>> = BinaryHeap::new();
    let mut rooms = 0;
    let mut out = vec![0i64; meetings.len()];
    for i in order {
        let (s, e) = (meetings[i][0], meetings[i][1]);
        while let Some(&Reverse((end, room))) = busy.peek() {
            if end > s {
                break;
            }
            busy.pop();
            free.push(Reverse(room));
        }
        let room = match free.pop() {
            Some(Reverse(r)) => r,
            None => {
                rooms += 1;
                rooms
            }
        };
        out[i] = room;
        busy.push(Reverse((e, room)));
    }
    out
}
