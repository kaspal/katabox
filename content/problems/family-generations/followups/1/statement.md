With the same kind of family tree, the society now wants to know how closely people are related. The **distance** between two people is the number of parent links on the way from one to the other through the tree: go up from each of them to the first ancestor they share (which may be one of the two), and add the two numbers of steps.

For each `pairs[i] = [a, b]`, return the distance between `a` and `b`, in order. A person is at distance 0 from themselves, and a parent and their child are at distance 1.

### Example 1

```
parent = [-1, 0, 0, 1, 1, 2], pairs = [[3, 4], [3, 5], [0, 0], [5, 2]]
Output: [2, 4, 0, 1]
```

Persons 3 and 4 share their parent 1. Persons 3 and 5 only meet at the founder, 2 steps up from each.

### Example 2

```
parent = [3, 3, -1, 2, 0, 3], pairs = [[0, 3], [0, 3], [3, 4]]
Output: [1, 1, 2]
```

The same pair may be asked more than once.

### Example 3

```
parent = [5, 3, -1, 2, 1, 3], pairs = [[0, 4], [5, 1], [3, 5]]
Output: [4, 2, 1]
```

### Constraints

- 1 ≤ n ≤ 10⁵, where n = parent.length
- Exactly one `parent[i]` is `-1`; every other is between `0` and `n − 1`
- Following `parent` from anyone always reaches the founder (there are no loops)
- 1 ≤ pairs.length ≤ 10⁵
- `pairs[i] = [a, b]` with 0 ≤ a, b < n (a may equal b)
