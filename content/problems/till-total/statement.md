Wen helps out at her uncle's corner shop and wrote a function that adds up the day's sales at closing time. Most evenings the till matches, but every so often the total is a cent short and her uncle counts the drawer twice.

Each entry of `prices` is a price written as a string with exactly two decimals, such as `"12.30"`. Return the total of all prices in **cents**, as an integer (`"12.30"` alone gives `1230`).

The editor starts with that code. Find the bug and fix it.

### Example 1

```
prices = ["0.10", "0.20", "4.35"]
Output: 465
```

10 + 20 + 435 = 465 cents.

### Example 2

```
prices = ["5.17","2.01"]
Output: 718
```

### Example 3

```
prices = ["4.35","25.25"]
Output: 2960
```

### Constraints

- 1 ≤ prices.length ≤ 10⁴
- Each price has the form `12.30`: a whole part from 0 to 9999 with no leading zeros, a dot, and exactly two digits (from `"0.00"` to `"9999.99"`)
