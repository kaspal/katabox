## The bug

The starter begins with `acc = 1.0`, so the whole computation runs on floating-point numbers. After each step `acc` is below 10⁹ + 7, but `acc * x` can be close to 10¹⁸, far beyond 2⁵³, the largest range in which a double stores every integer exactly. The product is rounded to the nearest representable value, and the remainder is taken from that wrong number.

With `nums = [999999999, 999999999]` the exact product is 999 999 998 000 000 001. As a double it becomes 999 999 998 000 000 000 (doubles near 10¹⁸ are 128 apart), so the starter returns 63 instead of 64. The TypeScript starter fails the same way, because `number` is a double too.

## The fix

Keep the product an integer; Python integers are exact at any size:

```python
def product_mod(nums: list[int]) -> int:
    acc = 1
    for x in nums:
        acc = acc * x % MOD
    return acc
```

In TypeScript, use `BigInt` for the multiplication (`acc = (acc * BigInt(x)) % 1000000007n`, starting from `1n`) and return `Number(acc)`, which is exact because the result is below 10⁹ + 7.

**Complexity:** O(n) time, O(1) extra space.
