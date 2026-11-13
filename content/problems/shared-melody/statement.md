A music app compares two tunes, written as lists of note numbers, `first` and `second`. A **shared melody** is a sequence of notes that appears in both tunes in the same order, though not necessarily next to each other (other notes may come in between).

Return the length of the longest shared melody.

### Example 1

```
first = [60, 62, 64, 65, 67], second = [62, 65, 60, 67]
Output: 3
```

62, 65, 67 appears in both, in that order.

### Example 2

```
first = [1, 2, 3], second = [4, 5]
Output: 0
```

### Example 3

```
first = [7, 7, 7], second = [7, 7]
Output: 2
```

### Constraints

- 1 ≤ first.length, second.length ≤ 1500
- 0 ≤ note number ≤ 127
