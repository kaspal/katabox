This time songs may repeat as often as they like, but the stretch has to stick to a short rotation: at most `k` **different** songs. Find the longest run of consecutive plays that uses no more than `k` distinct song numbers.

Return its length.

### Example 1

```
plays = [7, 3, 7, 7, 3, 9, 3], k = 2
Output: 5
```

The plays 7, 3, 7, 7, 3 (positions 0 to 4) use only songs 7 and 3.

### Example 2

```
plays = [2, 1, 3, 1, 4, 4, 4, 4], k = 2
Output: 5
```

The plays 1, 4, 4, 4, 4 (positions 3 to 7).

### Example 3

```
plays = [1, 1, 1, 3, 2, 3, 3, 2], k = 100000
Output: 8
```

With `k` this large, the whole log qualifies.

### Constraints

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
- 1 ≤ k ≤ 10⁵
