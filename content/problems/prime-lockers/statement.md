A school numbers its lockers 1, 2, 3, … along one very long corridor. For a festival, every locker whose number is a **prime** gets painted blue. (A prime has exactly two divisors, 1 and itself, so locker 1 stays grey.)

Teachers ask many questions of the same kind: "how many blue lockers are there from locker 1 up to locker `L`?" `limits` lists the `L` of every question. Return the answers in the same order.

### Example 1

```
limits = [10, 2, 30]
Output: [4, 1, 10]
```

Up to 10 the blue lockers are 2, 3, 5 and 7.

### Example 2

```
limits = [1]
Output: [0]
```

### Example 3

```
limits = [100, 100, 97]
Output: [25, 25, 25]
```

### Constraints

- 1 ≤ limits.length ≤ 2·10⁴
- 1 ≤ limits[i] ≤ 10⁶
