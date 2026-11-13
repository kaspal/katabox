pub fn sort_bins(items: Vec<i64>) -> Vec<i64> {
    let mut items = items;
    let (mut low, mut mid, mut high) = (0usize, 0usize, items.len());
    // `high` is one past the unknown region here, so it never goes below zero.
    while mid < high {
        match items[mid] {
            0 => {
                items.swap(low, mid);
                low += 1;
                mid += 1;
            }
            1 => mid += 1,
            _ => {
                high -= 1;
                items.swap(mid, high);
            }
        }
    }
    items
}
