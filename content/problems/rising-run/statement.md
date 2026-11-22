Ingrid logs the altitude at every checkpoint of her mountain runs and likes to know the longest climb she could pick out of a route. She found a fast trick online and coded it, but on some routes her function brags about climbs that never happened.

`heights` lists the checkpoint altitudes in the order she passes them. Pick some of the checkpoints, keeping their order, so that each picked checkpoint is **strictly higher** than the one picked before it. Return the largest number of checkpoints such a pick can have.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
heights = [3, 1, 3, 3, 4, 2, 5]
Output: 4
```

For instance 1, 3, 4, 5. Two checkpoints at the same height cannot both be picked.

### Example 2

```
heights = [5,1,3,1,4]
Output: 3
```

### Example 3

```
heights = [1,1,3,2,3]
Output: 3
```

### Constraints

- 1 ≤ heights.length ≤ 10⁵
- 1 ≤ heights[i] ≤ 10⁹
