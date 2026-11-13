use std::collections::BinaryHeap;

pub fn fuel_stops(target: i64, fuel: i64, stations: Vec<Vec<i64>>) -> i64 {
    let mut heap = BinaryHeap::new();
    let (mut reach, mut stops) = (fuel, 0);
    let stops_list = stations.iter().map(|s| (s[0], s[1])).chain(std::iter::once((target, 0)));
    for (position, litres) in stops_list {
        while reach < position {
            match heap.pop() {
                Some(x) => {
                    reach += x;
                    stops += 1;
                }
                None => return -1,
            }
        }
        heap.push(litres);
    }
    stops
}
