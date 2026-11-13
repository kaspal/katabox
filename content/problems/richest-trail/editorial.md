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

`[-5, 4, 8, null, null, -2, 6]`: gain(−2) = −2, gain(6) = 6, gain(8) = 8 + 6 = 14, gain(4) = 4. The bend at 8 is 8 + 0 + 6 = 14; the bend at −5 is −5 + 4 + 14 = 13. The answer is 14.

**Complexity:** O(n) time, O(h) for the recursion.
