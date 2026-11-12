## Approach: top-down order, then add bottom-up

The team total of employee `e` is `sales[e]` plus the team totals of everyone `e` leads directly. That suggests computing the totals **from the bottom up**, but the numbering says nothing about who is above whom.

1. Build children lists from `boss` and find the owner.
2. Walk from the owner level by level (breadth-first). The resulting order lists every boss **before** everyone in their team.
3. Start with `total = sales`, and go through the order **backwards**: add `total[e]` into `total[boss[e]]`. When `e` is reached, everyone below `e` comes later in the order and has already added their total into `e`, so `total[e]` is final when it is passed up.

```
order = BFS from the owner
total = copy of sales
for e in reversed(order):
    if boss[e] >= 0: total[boss[e]] += total[e]
```

### Walkthrough

`boss = [-1, 0, 0, 1, 1]`: the order is 0, 1, 2, 3, 4. Backwards: 4 adds 1 to employee 1 (now 4), 3 adds 2 (now 6), 2 adds 4 to the owner (9), 1 adds 6 (15). Totals: 15, 6, 4, 2, 1.

**Complexity:** O(n) time and memory. Adding each employee's sales to all of their bosses is O(n · depth), up to about n²/2 in a single chain of command.
