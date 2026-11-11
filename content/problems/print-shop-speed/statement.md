A print shop must finish a list of jobs before it closes; `pages[i]` is the length of job `i`. Its printer can be set to a whole number of pages per hour, `speed`. It prints one job at a time, and an hour is never shared between two jobs: a job of `p` pages takes ⌈p / speed⌉ hours, and the next job starts at the following hour.

Return the smallest `speed` that finishes all jobs within `hours` hours. It always exists, since `hours` is at least the number of jobs.

### Example 1

```
pages = [7, 3, 12], hours = 6
Output: 4
```

At speed 4 the jobs take 2 + 1 + 3 = 6 hours. At speed 3 they take 3 + 1 + 4 = 8.

### Example 2

```
pages = [10, 10], hours = 2
Output: 10
```

### Example 3

```
pages = [5], hours = 100
Output: 1
```

### Constraints

- 1 ≤ pages.length ≤ 5·10⁴
- 1 ≤ pages[i] ≤ 10⁹
- pages.length ≤ hours ≤ 10⁹
