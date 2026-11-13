A lock factory gives each key a numeric code. Two keys are a good pair for a master system when their codes differ in as many high bits as possible: the **XOR** of the two codes (bitwise exclusive or) should be as large as possible.

Given the `codes`, return the largest value of `codes[i] XOR codes[j]` over two different keys `i ≠ j`.

### Example 1

```
codes = [3, 10, 5, 25, 2, 8]
Output: 28
```

5 XOR 25 = 00101 XOR 11001 = 11100 = 28.

### Example 2

```
codes = [0, 0]
Output: 0
```

### Example 3

```
codes = [7, 8]
Output: 15
```

### Constraints

- 2 ≤ codes.length ≤ 10⁵
- 0 ≤ codes[i] < 2²⁰ (that is, below 1 048 576)
