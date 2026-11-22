The librarian's shelf is the same as in the base problem: folders with different
catalogue numbers, sorted from smallest to largest, after a helper moved a block from
the **start** to the **end** in the same order (maybe none, maybe all but one). `shelf`
lists the numbers as they stand.

Now a whole list of requests comes in. Return an array whose `k`-th entry is the
position of the folder numbered `labels[k]`, or `-1` if it is not on the shelf. A
number may be requested more than once.

### Example 1

```
shelf = [41, 57, 63, 8, 15, 22], labels = [15, 40, 41, 22]
Output: [4, -1, 0, 5]
```

### Example 2

```
shelf = [48, 51, 54, 4, 36], labels = [4, 4, 6]
Output: [3, 3, -1]
```

### Example 3

```
shelf = [23, 53, 5, 10], labels = [53, 37, 27]
Output: [1, -1, -1]
```

### Constraints

- 1 ≤ shelf.length, labels.length ≤ 10⁵
- 0 ≤ shelf[i], labels[k] ≤ 10⁹
- The numbers on the shelf are all different, and `shelf` is a sorted list with a block
  moved from the start to the end.

Scanning the shelf for every label is up to 10¹⁰ steps; aim for O(log n) per label.
