## Approach: XOR cancels pairs

XOR has three properties that solve the problem at once:

- `x ^ x = 0`: a number cancels itself;
- `x ^ 0 = x`: zero changes nothing;
- it is commutative and associative, so the order of the socks does not matter.

XOR every id into an accumulator that starts at 0. The pairs vanish and the accumulator ends as the lonely id.

### Walkthrough

`ids = [14, 3, 9, 3, 14]`:

| id | accumulator |
| --- | --- |
| | 0 |
| 14 | 14 |
| 3 | 14 ^ 3 |
| 9 | 14 ^ 3 ^ 9 |
| 3 | 14 ^ 9 |
| 14 | 9 |

**Complexity:** O(n) time, O(1) extra space. Searching for each sock's partner is O(n²), too slow for 10⁵ socks; a hash map of counts is O(n) but needs O(n) memory.
