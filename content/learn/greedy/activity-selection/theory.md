## The idea

You have a list of activities, each with a start and an end time, and one room. Two activities clash if one starts before the other ends; touching is fine, so an activity ending at 6 and one starting at 6 can both happen. What is the **largest number** of activities that fit?

A **greedy** algorithm builds its answer one choice at a time and never takes a choice back. Here the choice is: among the activities still possible, take the one that **ends earliest**. The sample sorts the activities by end (ties by start, then by input position), walks through them once, and takes each one that starts no earlier than the last taken one ended. The output is the count, then the chosen input positions (0-based) in ascending order. The input is typed as "start end" pairs.

Why is the earliest end safe? Take any best schedule and look at its first activity. Swap it for the earliest-ending one: that one finishes no later, so it cannot clash with anything that came after. The count stays the same, and what remains is the same problem on a smaller list. Other rules break on tiny examples. **Shortest first** fails on `(0, 5)`, `(4, 7)`, `(6, 11)`: it takes `(4, 7)`, which clashes with both others, so it gets 1 instead of 2. **Earliest start** fails on `(0, 10)`, `(1, 3)`, `(4, 6)`: the long one comes first and blocks the two short ones.

A problem is greedy-friendly when it has a **greedy choice** (some local pick is always part of a best answer) and **optimal substructure** (after that pick, the rest is a smaller copy of the same problem). Distrust every greedy idea until you have tried to break it with a small example of two or three items. If you find a break, the problem probably needs dynamic programming.

### The default input

Eight activities: `(1, 4)`, `(3, 5)`, `(0, 10)`, `(5, 7)`, `(4, 6)`, `(6, 9)`, `(8, 11)`, `(9, 12)`, positions 0 to 7. By end the order is positions 0, 1, 4, 3, 5, 2, 6, 7. Position 0 is taken (free at 4). Position 1 starts at 3: clash. Position 4 starts at 4: taken (free at 6). Position 3 clashes, position 5 starts at 6: taken (free at 9). Positions 2 and 6 clash, and position 7 starts at 9: taken.

The output is `[4, 0, 4, 5, 7]`. Earliest start would grab the long meeting `(0, 10)` and fit nothing else.

## Steps

1. Sort the positions by end, then start, then position.
2. Keep `free_at`, the end of the last taken activity (none at first).
3. For each activity in that order: if `start ≥ free_at`, take it and set `free_at` to its end; otherwise skip it.
4. Return the count and the taken positions, sorted.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n log n)`: the sort; the walk itself is `O(n)` |
| Extra space | `O(n)` for the sorted order and the chosen list |

## Pitfalls

- **Sorting by start.** It looks natural but lets one long activity block many short ones.
- **Strict or not.** Here touching is allowed (`start ≥ free_at`); if a problem says an activity needs a gap, use `>`.
- **Weighted activities.** When each activity has a value and the goal is the largest total value, the earliest end no longer wins; that version needs dynamic programming.

## When to use it

Use it to fit the most meetings, talks or jobs into one room or machine. The linked problem, Most Talks, is this pattern.

## Related topics

- [Merge intervals](learn:greedy/merge-intervals): the same sorted intervals, joined instead of picked.
- [Coin change](learn:dynamic-programming/coin-change): a greedy rule that breaks, and the table that fixes it.
- [0/1 knapsack](learn:dynamic-programming/knapsack-01): where "best value per weight first" fails.
- [Greedy best-first search](learn:grid-pathfinding/greedy-best-first): a greedy search that is fast but not always cheapest.
