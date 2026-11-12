A family history society has drawn a family tree with `n` people, numbered `0` to `n − 1`. For each person, `parent[i]` is the number of the person they descend from directly, or `-1` for the one founder at the top. Everybody descends from the founder.

The founder is the first generation, their children the second, and so on. How many **generations** does the tree have?

### Example 1

```
parent = [-1, 0, 0, 1, 1, 4]
Output: 4
```

Generation 1: person 0. Generation 2: persons 1 and 2. Generation 3: persons 3 and 4. Generation 4: person 5.

### Example 2

```
parent = [-1]
Output: 1
```

### Example 3

```
parent = [2, -1, 1]
Output: 3
```

The founder is person 1, their child is person 2, and person 0 is the child of person 2. The numbers do not follow the generations.

### Constraints

- 1 ≤ n ≤ 10⁵
- Exactly one `parent[i]` is `-1`; every other is between `0` and `n − 1`
- Following `parent` from anyone always reaches the founder (there are no loops)
