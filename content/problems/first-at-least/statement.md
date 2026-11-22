Hana keeps the weights of her bakery orders in a sorted list, and a binary search tells her where the orders of at least a given weight begin. It answers most questions in a blink, but for some weights it points at the wrong spot.

Given an array `sorted` in non-decreasing order and an integer `x`, return the smallest index `i` such that `sorted[i] ≥ x`. If no element is at least `x`, return `sorted.length`.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
sorted = [2, 5, 5, 9], x = 5
Output: 1
```

Index 1 holds the first 5.

### Example 2

```
sorted = [-16, -13, -4, 11, 16], x = 8
Output: 3
```

### Example 3

```
sorted = [-15, -15, -10, -1, 3], x = 5
Output: 5
```

Every element is smaller than 5, so the answer is the length of the array.

### Constraints

- 1 ≤ sorted.length ≤ 10⁵
- −10⁹ ≤ sorted[i], x ≤ 10⁹
- sorted is in non-decreasing order.
