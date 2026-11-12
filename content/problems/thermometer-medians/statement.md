A weather station logs one temperature reading at a time, in tenths of a degree, and after every reading its display shows the **median** of all readings so far. The median is the middle value of the sorted readings; with an even number of readings it is the average of the two middle values.

Given the `readings` in the order they arrive, return the median shown after each one.

Answers within 10⁻⁶ of the exact value are accepted.

### Example 1

```
readings = [12, -4, 30, 7]
Output: [12.0, 4.0, 12.0, 9.5]
```

After `12, -4` the middle values are −4 and 12, so the median is 4. After all four readings, sorted `−4, 7, 12, 30`, it is (7 + 12) / 2 = 9.5.

### Example 2

```
readings = [5, 5, 5]
Output: [5.0, 5.0, 5.0]
```

### Example 3

```
readings = [1, 2]
Output: [1.0, 1.5]
```

### Constraints

- 1 ≤ readings.length ≤ 10⁵
- −10⁴ ≤ readings[i] ≤ 10⁴
