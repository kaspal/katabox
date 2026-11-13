A delivery van logs how many kilometres it drives each day: `miles[i]` for day `i` (days count from 0). The accountant asks about many **trips**: each trip `[a, b]` covers days `a` to `b`, both included.

Return the total kilometres of each trip, in the order asked.

### Example 1

```
miles = [4, 0, 7, 3, 5], trips = [[0, 2], [1, 3], [4, 4], [0, 4]]
Output: [11, 10, 5, 19]
```

Days 0 to 2 give 4 + 0 + 7 = 11.

### Example 2

```
miles = [9], trips = [[0, 0], [0, 0]]
Output: [9, 9]
```

### Example 3

```
miles = [2, 2, 2, 2], trips = [[1, 2]]
Output: [4]
```

### Constraints

- 1 ≤ miles.length ≤ 10⁵
- 0 ≤ miles[i] ≤ 1000
- 1 ≤ trips.length ≤ 10⁵
- 0 ≤ a ≤ b < miles.length for each trip [a, b]
