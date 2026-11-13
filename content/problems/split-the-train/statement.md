A freight train, `train`, is a linked list of wagon numbers from the engine backwards. At the junction it must be split into exactly `k` consecutive parts, one per track:

- the parts keep the wagons in order, and together they hold every wagon;
- their lengths differ by at most one, and no part is shorter than a later part.

Some parts may be empty when there are fewer wagons than tracks. Return the `k` parts in order.

Lists are written as their values in order, so `[1, 4, 9]` is the list 1 → 4 → 9, and `[]` is the empty list.

### Example 1

```
train = [1, 2, 3, 4, 5, 6, 7], k = 3
Output: [[1, 2, 3], [4, 5], [6, 7]]
```

7 wagons on 3 tracks: lengths 3, 2, 2.

### Example 2

```
train = [8, 9], k = 4
Output: [[8], [9], [], []]
```

### Example 3

```
train = [], k = 2
Output: [[], []]
```

### Constraints

- 0 ≤ number of wagons ≤ 10⁴
- 1 ≤ wagon number ≤ 10⁶
- 1 ≤ k ≤ 100
