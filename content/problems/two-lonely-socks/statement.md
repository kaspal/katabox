The laundry room is back, and this time **two** socks lost their partners. The machine lists the number stitched in every sock it found, `ids`: every number appears exactly twice, except two different numbers that appear once each.

Return the two lonely numbers in **increasing order**.

Can you do it with one pass over the list and a constant amount of extra memory, without sorting or a hash map?

### Example 1

```
ids = [8, 5, 13, 5, 2, 8]
Output: [2, 13]
```

### Example 2

```
ids = [41, 40]
Output: [40, 41]
```

### Example 3

```
ids = [6, 1, 6, 9, 1, 3]
Output: [3, 9]
```

### Constraints

- 2 ≤ ids.length ≤ 10⁵ + 2
- 1 ≤ ids[i] ≤ 10⁹
- Every number appears exactly twice, except two different numbers that appear once
