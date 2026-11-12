A bakery logged how many loaves it sold each day, in order, in `sales`. For its anniversary poster it wants to brag about its best run of `k` **consecutive** days.

Return the largest total of loaves sold over any `k` consecutive days.

### Example 1

```
sales = [12, 30, 7, 25, 18, 4], k = 3
Output: 62
```

Days 1 to 3 sold 30 + 7 + 25 = 62; no other run of three days sold more.

### Example 2

```
sales = [9, 0, 9], k = 1
Output: 9
```

### Example 3

```
sales = [5, 6, 2], k = 3
Output: 13
```

With `k` equal to the number of days, the only run is the whole log.

### Constraints

- 1 ≤ k ≤ sales.length ≤ 2·10⁵
- 0 ≤ sales[i] ≤ 10⁴
