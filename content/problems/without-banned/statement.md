Marco moderates a sticker-trading forum and filters out sticker ids that have been banned. His filter clearly removes some banned ids, yet every so often one of them slips through into the final list.

Given an array `items` and an array `banned`, return the elements of `items` that do not appear in `banned`, keeping their original order. Duplicates of an allowed value are all kept.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
items = [4, 7, 7, 2, 9, 7], banned = [7]
Output: [4, 2, 9]
```

Every 7 is removed.

### Example 2

```
items = [49, 5, 17, 8, 32, 49], banned = [31, 42, 25]
Output: [49, 5, 17, 8, 32, 49]
```

Nothing in `items` is banned.

### Example 3

```
items = [1, 2, 1, 3, 3, 2], banned = [3, 1]
Output: [2, 2]
```

### Constraints

- 1 ≤ items.length ≤ 10⁵
- 0 ≤ banned.length ≤ 10⁵
- 1 ≤ items[i], banned[j] ≤ 10⁹
