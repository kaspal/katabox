## Approach: best downward gain, best bend at each node

Every trail has a single highest cave `v`; from there it may go down into the left subtree, into the right subtree, both, or neither.

Define `gain(v)` = the best total of a downward trail starting at `v`:

```
gain(v) = val(v) + max(0, gain(left), gain(right))
```

(a missing child contributes 0; a negative branch is simply not taken). The best trail bending at `v` is

```
val(v) + max(0, gain(left)) + max(0, gain(right))
```

A post-order walk computes `gain` for every node and keeps the maximum bend. The answer starts at −∞: with all values negative, the best trail is the single least negative cave.

### Walkthrough

`[6, -8, 5, 3, null, 4, -2]`: gain(3) = 3, gain(−8) = −8 + 3 = −5, gain(4) = 4, gain(−2) = −2, gain(5) = 5 + 4 = 9. The bend at 5 is 5 + 4 + 0 = 9; the bend at 6 is 6 + max(0, −5) + max(0, 9) = 15. The answer is 15.

**Complexity:** O(n) time, O(h) for the recursion.
