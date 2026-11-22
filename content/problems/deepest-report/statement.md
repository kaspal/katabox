Rosa works in HR at a fast-growing company and wanted to know how many levels separate the CEO from the most junior employee. Her function answers instantly for every department she tried, but when she fed it the full company chart it crashed instead of answering.

There are `manager.length` people numbered from 0. `manager[i]` is the person that `i` reports to, and exactly one person, the boss, has `manager[i] = -1`. Everyone's chain of managers leads up to the boss. The boss is at level 1, and anyone who reports to a person at level `d` is at level `d + 1`. Return the deepest level in the company.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
manager = [2, 2, -1, 0, 3, 1]
Output: 4
```

Person 2 is the boss (level 1), 0 and 1 report to 2 (level 2), 3 and 5 are at level 3, and 4 reports to 3, so it is at level 4.

### Example 2

```
manager = [3,3,-1,2]
Output: 3
```

### Example 3

```
manager = [1,2,-1,1]
Output: 3
```

### Constraints

- 1 ≤ manager.length ≤ 5·10⁴
- −1 ≤ manager[i] < manager.length
- Exactly one person (the boss) has manager −1
- Every chain of managers reaches the boss (there are no cycles)
