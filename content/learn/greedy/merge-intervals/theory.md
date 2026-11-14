## The idea

A list of intervals, such as room bookings, may overlap. Merging them means replacing every group of overlapping intervals with one interval that covers the same stretch, so the result has no overlaps left. Here intervals that only touch, where one ends exactly where the next starts, are merged too.

The trick is to **sort by start** (ties by end). After sorting, an interval can only overlap the one merged just before it: anything earlier ended before that one began. So a single sweep is enough. Keep a list of merged intervals. For each interval in sorted order, if it starts **no later than the last merged one ends**, it belongs to that one: **extend** the end to the larger of the two ends. Otherwise there is a gap, and the interval **opens** a new merged one.

The greedy choice is to commit to a merged interval as soon as a gap appears; no later interval can reach back over that gap, because later ones start even further right. The output lists the merged intervals flat, start then end. The input is typed as "start end" pairs.

### The default input

Six intervals: `(7, 9)`, `(1, 4)`, `(3, 5)`, `(14, 17)`, `(8, 11)`, `(5, 6)`. Sorted: `(1, 4)`, `(3, 5)`, `(5, 6)`, `(7, 9)`, `(8, 11)`, `(14, 17)`. `(1, 4)` opens. `(3, 5)` starts at 3 ≤ 4: extend to 5. `(5, 6)` touches at 5: extend to 6. `(7, 9)` starts after 6: open. `(8, 11)` starts at 8 ≤ 9: extend to 11. `(14, 17)` opens.

The output is `[1, 6, 7, 11, 14, 17]`. `(1, 6)` and `(7, 11)` stay apart: nothing covers the gap from 6 to 7.

## Steps

1. Sort the intervals by start, then end.
2. For each interval `(start, end)`: if the list is not empty and `start ≤` the last merged end, set that end to `max(last end, end)`.
3. Otherwise append `(start, end)` as a new merged interval.
4. Return the merged intervals, flattened.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n log n)`: the sort; the sweep is `O(n)` |
| Extra space | `O(n)` for the sorted copy and the merged list |

## Pitfalls

- **Overwriting instead of taking the max.** In `(1, 10)`, `(2, 3)` the second interval sits inside the first; setting the end to 3 would lose most of the first one.
- **Skipping the sort.** Unsorted, an interval may overlap one merged long before, and the sweep never looks back.
- **The touching rule.** Whether `(1, 4)` and `(4, 6)` merge depends on the problem. Use `≤` when they should join, `<` when a shared end point does not count as overlap.

## When to use it

Use it to tidy calendars, join busy times, or find what a set of ranges covers. The linked problems apply the same sweep. Merge Bookings is this pattern directly. In Insert a Slot the list is already sorted, so the new slot can be placed in order and the sweep run once without a sort. In Paint Strokes, sum the lengths of the merged intervals so that paint applied twice counts once.

## Related topics

- [Activity selection](learn:greedy/activity-selection): the same intervals, sorted by end, picking the most that do not clash.
- [Prefix sums](learn:foundations/prefix-sums): add 1 at each start and subtract 1 after each end, and a running sum counts how many intervals cover each point.
- [Two pointers](learn:searching/two-pointers): walking two sorted lists of intervals at once, for example to find where they overlap.
