A shop's ledger lists one amount per line in `nums`: sales are positive, refunds are negative. The auditor knows that exactly two lines together come to `target` and wants to find them.

Return the positions (0-based) of those two lines, in any order. A line can be used only once, even if its amount alone is half of `target`.

### Example 1

```
nums = [4, 11, 7, 15, 2], target = 13
Output: [1, 4]
```

Lines 1 and 4 hold 11 and 2, which make 13. No other pair does.

### Example 2

```
nums = [-6, 14, 3, 20, -1], target = 14
Output: [0, 3]
```

A refund of 6 and a sale of 20 come to 14. The line holding 14 cannot pair with itself.

### Constraints

- 2 ≤ nums.length ≤ 10⁵
- −10⁹ ≤ nums[i], target ≤ 10⁹
- Exactly one pair of lines comes to `target`.
