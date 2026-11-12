A cookbook's big recipe has `n` steps, numbered `0` to `n − 1`. Each pair `[a, b]` in `before` says that step `a` must be finished before step `b` can start. The cook works through the steps one at a time and, whenever several steps are ready, always does the **lowest-numbered** one.

Return the steps in the order the cook does them. If the rules contradict each other, so that some steps can never start, return an **empty** list.

### Example 1

```
n = 4, before = [[0, 2], [1, 2], [3, 1]]
Output: [0, 3, 1, 2]
```

At first steps 0 and 3 are ready; the cook does 0, then 3. That makes 1 ready, and after 1, step 2.

### Example 2

```
n = 3, before = [[0, 1], [1, 2], [2, 0]]
Output: []
```

Each step waits for another one in a circle.

### Example 3

```
n = 3, before = []
Output: [0, 1, 2]
```

### Constraints

- 1 ≤ n ≤ 2·10⁴
- 0 ≤ before.length ≤ 4·10⁴
- `before[i] = [a, b]` with 0 ≤ a, b < n and a ≠ b; a pair may repeat
