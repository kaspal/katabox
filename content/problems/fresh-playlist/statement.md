A radio station logs every song it plays, by song number, in `plays`. The station manager wants to advertise its longest **fresh stretch**: a run of consecutive plays in which no song was played twice.

Return the length of the longest fresh stretch.

### Example 1

```
plays = [4, 8, 4, 2, 9, 8, 1]
Output: 5
```

The plays 4, 2, 9, 8, 1 (positions 2 to 6).

### Example 2

```
plays = [7, 7, 7]
Output: 1
```

### Example 3

```
plays = [3, 1, 2]
Output: 3
```

### Constraints

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
