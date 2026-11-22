The van's log is the same: `miles[i]` kilometres on day `i` (days count from 0). This
time the accountant checks the drivers' rest rules and asks about **long days**: each
question `[a, b, x]` asks how many days from `a` to `b` (both included) the van drove **at
least** `x` kilometres.

Return the answer to each question, in the order asked.

### Example 1

```
miles = [5, 12, 0, 30, 7], trips = [[0, 4, 7], [1, 3, 13], [2, 2, 0]]
Output: [3, 1, 1]
```

Days 1, 3 and 4 drove at least 7 km; among days 1 to 3 only day 3 reached 13 km; day 2
drove 0 km, which is at least 0.

### Example 2

```
miles = [100, 100, 100], trips = [[0, 2, 100], [0, 2, 101], [1, 1, 50]]
Output: [3, 0, 1]
```

### Constraints

- 1 ≤ miles.length ≤ 10⁵
- 0 ≤ miles[i] ≤ 1000
- 1 ≤ trips.length ≤ 10⁵
- `trips[j] = [a, b, x]` with 0 ≤ a ≤ b < miles.length and 0 ≤ x ≤ 1000
