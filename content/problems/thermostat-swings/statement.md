A greenhouse thermostat logs one temperature reading per minute in `readings`. To spot unstable periods, the gardener looks at every stretch of `k` consecutive readings and measures its **swing**: the highest reading in the stretch minus the lowest.

Return the swing of every stretch of `k` readings, from the first stretch (readings 0 to k − 1) to the last.

### Example 1

```
readings = [3, 1, 4, 1, 5, 9, 2, 6], k = 3
Output: [3, 3, 4, 8, 7, 7]
```

The first stretch [3, 1, 4] swings 4 − 1 = 3; the stretch [1, 5, 9] swings 8.

### Example 2

```
readings = [5], k = 1
Output: [0]
```

### Example 3

```
readings = [2, 2, 2, 2], k = 2
Output: [0, 0, 0]
```

### Constraints

- 1 ≤ k ≤ readings.length ≤ 10⁵
- −10⁴ ≤ readings[i] ≤ 10⁴
