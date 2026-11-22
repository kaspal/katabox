This time each talk has a **score**: how much the visitor wants to see it. Talk `i` is
`talks[i] = [start, end, score]`. The visitor still attends whole talks only, one at a
time, and a talk that ends at a moment leaves time to start another talk at that same
moment.

Return the **largest total score** of a set of talks the visitor can attend. The number
of talks no longer matters, only their total score.

### Example 1

```
talks = [[1, 4, 5], [3, 6, 9], [4, 7, 5], [6, 9, 2]]
Output: 11
```

3–6 and 6–9 give 9 + 2 = 11, better than 1–4 and 4–7 (5 + 5 = 10).

### Example 2

```
talks = [[2, 7, 3], [4, 5, 16], [7, 11, 13], [3, 4, 16]]
Output: 45
```

3–4, 4–5 and 7–11 give 16 + 16 + 13 = 45.

### Example 3

```
talks = [[0, 1, 3], [5, 7, 10], [4, 9, 7], [9, 10, 19]]
Output: 32
```

### Constraints

- 1 ≤ talks.length ≤ 10⁵
- talks[i] = [start, end, score] with 0 ≤ start < end ≤ 10⁶ and 1 ≤ score ≤ 10⁴
