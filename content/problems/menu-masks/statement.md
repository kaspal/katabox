A food truck sells a few dishes; `prices[i]` is the price of dish `i`. A customer wants to order a set of **different** dishes (at most one of each) that costs **exactly** `budget`.

Return every such set, each written as the list of its dish numbers in increasing order. Sort the list of sets lexicographically (compare first elements, then second elements, and so on; a shorter list that is a prefix of a longer one comes first). Return an empty list if there is none.

### Example 1

```
prices = [4, 2, 6, 3], budget = 9
Output: [[0, 1, 3], [2, 3]]
```

4 + 2 + 3 = 9 and 6 + 3 = 9.

### Example 2

```
prices = [5, 5], budget = 5
Output: [[0], [1]]
```

### Example 3

```
prices = [7], budget = 3
Output: []
```

### Constraints

- 1 ≤ prices.length ≤ 15
- 1 ≤ prices[i] ≤ 1000
- 1 ≤ budget ≤ 15000
