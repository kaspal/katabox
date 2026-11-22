This time some stages pay you back: a **refund** makes `costs[i]` negative, and the budget
itself may be negative too (you insist on coming out ahead). You still want to walk
**consecutive** stages whose total cost is at most `budget`.

Return the largest number of consecutive stages (at least one) whose total cost is at
most `budget`, or `0` if no such stretch exists.

### Example 1

```
costs = [5, -3, 4, 6, -8, 2], budget = 4
Output: 5
```

Stages 1 to 5 cost −3 + 4 + 6 − 8 + 2 = 1. All six stages cost 6, which is too much.

### Example 2

```
costs = [9, -7, -1, -6, 6, 5], budget = 6
Output: 6
```

### Example 3

```
costs = [-7, -7, 2, -4, 0, -1], budget = 10
Output: 6
```

### Constraints

- 1 ≤ costs.length ≤ 5·10⁴
- −10⁴ ≤ costs[i] ≤ 10⁴
- −10⁹ ≤ budget ≤ 10⁹
