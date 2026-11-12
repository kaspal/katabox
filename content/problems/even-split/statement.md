After a weekend trip, some friends want to share the costs **equally**. `paid[i]` is how much friend `i` spent on the group, in euros.

Return, for every friend, their balance: how much more than the fair share they paid. A positive balance means the others owe that friend money; a negative one means the friend still has to pay.

Answers within 10⁻⁶ of the exact value are accepted.

### Example 1

```
paid = [30.0, 10.5, 20.25, 0.0]
Output: [14.8125, -4.6875, 5.0625, -15.1875]
```

Together they spent 60.75, so the fair share is 15.1875 each.

### Example 2

```
paid = [12.0, 12.0]
Output: [0.0, 0.0]
```

### Example 3

```
paid = [8.0]
Output: [0.0]
```

A friend travelling alone owes nobody.

### Constraints

- 1 ≤ paid.length ≤ 2·10⁴
- 0 ≤ paid[i] ≤ 10⁴
