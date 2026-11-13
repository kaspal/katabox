A hiking app stores the profile of a mountain trail as the altitude at each marker, in `heights`. The trail always climbs strictly and then descends strictly: the altitudes strictly increase up to the summit and strictly decrease after it (the climb or the descent may be empty, if the summit is at either end).

Return the index of the summit, the highest marker.

### Example 1

```
heights = [1, 4, 9, 7, 2]
Output: 2
```

### Example 2

```
heights = [5, 3, 1]
Output: 0
```

The trail starts at the summit.

### Example 3

```
heights = [2, 6]
Output: 1
```

### Constraints

- 1 ≤ heights.length ≤ 10⁵
- 0 ≤ heights[i] ≤ 10⁹
- heights strictly increases and then strictly decreases.

An O(n) scan also passes the tests; the exercise is the O(log n) search.
