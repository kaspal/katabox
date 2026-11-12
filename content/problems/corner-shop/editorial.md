## Approach: a hash map of units per item

Keep `shelf`, a hash map from product code to the units on the shelf; a missing code means 0 units. Each operation is then a constant-time update:

- `stock(item, qty)`: `shelf[item] += qty`.
- `sell(item, qty)`: `sold = min(qty, shelf[item])`; `shelf[item] -= sold`; report `sold`.
- `count(item)`: report `shelf[item]`.

Wrapping the map in a small class with one method per operation keeps the simulation readable, and the main loop just dispatches on `ops[i]`.

### Walkthrough

Example 1: after `stock 7 5` the shelf holds 5. `sell 7 3` sells 3 (2 left), `count 7` reports 2, `sell 7 4` sells only 2, and `count 7` reports 0. Reports: 3, 2, 2, 0.

**Complexity:** O(1) expected time per operation, O(ops) in total, and O(distinct items) memory.
