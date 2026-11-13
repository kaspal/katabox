A street market is a row of stalls; stall `i` sells one kind of product, written as the letter `stalls[i]`. Your shopping list `need` is a string of letters: each letter is one product you must buy, and a letter that appears several times means you must visit that many stalls of that kind (each stall sells you one item).

You want to walk along as few **consecutive** stalls as possible. Return the length of the shortest stretch of consecutive stalls where you can buy everything on the list, or `-1` if the market does not have enough stalls.

### Example 1

```
stalls = "bcaacbdab", need = "aab"
Output: 4
```

For example the stalls "bcaa" (positions 0 to 3). No 3 consecutive stalls have two `a` and a `b`.

### Example 2

```
stalls = "xyz", need = "xx"
Output: -1
```

### Example 3

```
stalls = "mango", need = "m"
Output: 1
```

### Constraints

- 1 ≤ stalls.length, need.length ≤ 10⁵
- Both strings have lowercase English letters only.
