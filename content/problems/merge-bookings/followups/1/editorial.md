## Idea

First combine each sheet as in the base problem: sort by start and extend the last range
while the next one starts at or before its end. Each room is now a sorted list of ranges
that share no moment. Then walk both lists with pointers `i` and `j`. The current ranges
overlap in `[max(a[i].start, b[j].start), min(a[i].end, b[j].end)]` when that start is at
most that end; record it. The range that ends first cannot overlap anything later in the
other list, so advance it. Each recorded piece lies inside one range of each room, so the
pieces come out sorted and already share no moment with each other.

## Complexity

O(n log n + m log m) time for sorting the two sheets, then O(n + m) for the walk; O(n + m)
extra memory for the combined lists and the output.
