Weather posts stand along a straight road; `posts[i]` is the kilometre mark of post `i` (two posts may share a mark). For every pair of posts, the gap is the distance between them. Sort the gaps of all n · (n − 1) / 2 pairs from smallest to largest.

Return the `k`-th gap in that order (counting from 1).

### Example 1

```
posts = [10, 2, 7, 3], k = 4
Output: 5
```

The six gaps are 8, 3, 7, 5, 1 and 4; sorted, 1, 3, 4, 5, 7, 8.

### Example 2

```
posts = [5, 5, 9], k = 1
Output: 0
```

### Example 3

```
posts = [0, 1000000], k = 1
Output: 1000000
```

### Constraints

- 2 ≤ posts.length ≤ 5·10⁴
- 0 ≤ posts[i] ≤ 10⁶
- 1 ≤ k ≤ n · (n − 1) / 2
