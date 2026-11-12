A bakery chain has `n` employees, numbered `0` to `n − 1`. `boss[i]` is the direct boss of employee `i`, or `-1` for the owner, who is everyone's boss in the end. Last month employee `i` sold `sales[i]` cakes.

An employee's **team** is themselves plus everyone below them: the people they lead directly, the people those people lead, and so on. For every employee, return the total sales of their team.

### Example 1

```
boss = [-1, 0, 0, 1, 1], sales = [5, 3, 4, 2, 1]
Output: [15, 6, 4, 2, 1]
```

Employee 1 leads 3 and 4, so their team sold 3 + 2 + 1 = 6. The owner's team is the whole chain: 5 + 6 + 4 = 15.

### Example 2

```
boss = [-1], sales = [7]
Output: [7]
```

### Example 3

```
boss = [1, -1, 0], sales = [1, 2, 3]
Output: [4, 6, 3]
```

The owner is employee 1, who leads 0, who leads 2.

### Constraints

- 1 ≤ n ≤ 10⁵, sales.length = boss.length = n
- Exactly one `boss[i]` is `-1`; every other is between `0` and `n − 1`
- Following `boss` from anyone always reaches the owner (there are no loops)
- 0 ≤ sales[i] ≤ 10⁴
