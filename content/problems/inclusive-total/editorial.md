## The bug

The loop header is wrong:

```python
for i in range(a, b):
```

Python's `range(a, b)` covers `a, a + 1, ..., b - 1`: the end is excluded. The problem asks for an inclusive range, so the last element `nums[b]` is always left out. With `nums = [5]`, `a = 0`, `b = 0`, the loop runs zero times and the code returns `0` instead of `5`. In Example 1 it returns -2 + 7 = 5 instead of 8.

## The fix

Extend the range by one so it includes `b`:

```python
def range_total(nums: list[int], a: int, b: int) -> int:
    total = 0
    for i in range(a, b + 1):
        total += nums[i]
    return total
```

In TypeScript the loop condition `i < b` becomes `i <= b`.

**Complexity:** O(b − a + 1) time, O(1) extra space.
