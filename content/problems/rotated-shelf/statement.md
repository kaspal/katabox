A librarian keeps one shelf of folders with different catalogue numbers, sorted from smallest to largest. One morning a helper took some folders from the **start** of the shelf and put them, in the same order, at the **end** (maybe none, maybe all but one). Now `shelf` lists the numbers as they stand.

Return the position of the folder numbered `label`, or `-1` if it is not on the shelf.

### Example 1

```
shelf = [41, 57, 63, 8, 15, 22], label = 15
Output: 4
```

The shelf was `[8, 15, 22, 41, 57, 63]` and the first three folders were moved to the end.

### Example 2

```
shelf = [41, 57, 63, 8, 15, 22], label = 40
Output: -1
```

### Example 3

```
shelf = [5], label = 5
Output: 0
```

### Constraints

- 1 ≤ shelf.length ≤ 10⁵
- 0 ≤ shelf[i], label ≤ 10⁹
- The numbers are all different, and `shelf` is a sorted list with a block moved from the start to the end.

An O(n) scan also passes the tests; the exercise is the O(log n) search.
