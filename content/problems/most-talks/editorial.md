## Approach: earliest end first

Sort the talks by end time. Keep `free`, the moment the visitor is free again (start at −∞). For each talk in order, if it starts at or after `free`, attend it and set `free` to its end.

**Why it is optimal:** take any best schedule. Its first talk ends no earlier than the talk that ends first overall, so swapping in the earliest-ending talk keeps the schedule valid and just as long. Repeating the argument on the remaining time gives the greedy schedule.

### Walkthrough

Example 1 sorted by end: 8–9, 9–11, 10–12, 11–13, 12–15. Attend 8–9 (free at 9), 9–11 (free at 11), skip 10–12, attend 11–13 (free at 13), skip 12–15: 3 talks.

**Complexity:** O(n log n) for the sort. A DP that, for every talk, scans all earlier talks for the best compatible one is O(n²).
