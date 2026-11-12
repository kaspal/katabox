## Approach: XOR, then split by one differing bit

`x XOR x = 0` and `x XOR 0 = x`, and XOR does not care about order. So XOR-ing the whole list cancels every pair and leaves `t = a XOR b`, the two lonely ids combined.

Since `a ≠ b`, `t` has at least one bit set, and in that bit `a` and `b` differ. Take the lowest one, `bit = t & -t`. Now XOR separately the ids that have this bit and the ids that do not. Both copies of a paired id fall into the same group and cancel, so one group's XOR is `a` and the other's is `b`.

```
t = XOR of all ids
bit = t & -t
a = XOR of the ids with (id & bit) != 0
b = t XOR a
return [min(a, b), max(a, b)]
```

### Walkthrough

`ids = [8, 5, 13, 5, 2, 8]`: `t = 13 XOR 2 = 1101₂ XOR 0010₂ = 1111₂`, so `bit = 1`. The odd ids are 5, 13, 5, whose XOR is 13; then `b = 15 XOR 13 = 2`. Answer `[2, 13]`.

**Complexity:** O(n) time and O(1) extra space. Looking for each id's partner by scanning the list is O(n²).
