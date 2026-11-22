This time the station allows an encore: a song may be played **twice** in a stretch, but not three times. Find the longest run of consecutive plays in which no song number appears more than twice.

Return its length.

### Example 1

```
plays = [7, 3, 7, 7, 3, 9, 3]
Output: 5
```

For example 3, 7, 7, 3, 9 (positions 1 to 5); any longer stretch plays 7 or 3 three times.

### Example 2

```
plays = [2, 1, 3, 1, 4, 4, 4, 4]
Output: 6
```

The plays 2, 1, 3, 1, 4, 4 (positions 0 to 5).

### Example 3

```
plays = [1, 1, 1, 3, 2, 3, 3, 2]
Output: 5
```

### Constraints

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
