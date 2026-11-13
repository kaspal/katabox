A bottling plant packs bottles in batches of `k`. Each full batch leaves the conveyor in reverse order (the last bottle packed comes out first), while a final batch with fewer than `k` bottles is shipped as it is.

The conveyor is a linked list of bottle numbers, `line`. Return the line after every full batch of `k` consecutive bottles has been reversed. Change the links, not the numbers stored in the nodes.

Lists are written as their values in order, so `[1, 4, 9]` is the list 1 → 4 → 9, and `[]` is the empty list.

### Example 1

```
line = [1, 2, 3, 4, 5, 6, 7, 8], k = 3
Output: [3, 2, 1, 6, 5, 4, 7, 8]
```

The last two bottles do not fill a batch.

### Example 2

```
line = [4, 9, 2], k = 1
Output: [4, 9, 2]
```

### Example 3

```
line = [4, 9, 2], k = 3
Output: [2, 9, 4]
```

### Constraints

- 0 ≤ number of bottles ≤ 10⁵
- 1 ≤ bottle number ≤ 10⁶
- 1 ≤ k ≤ 10⁵
