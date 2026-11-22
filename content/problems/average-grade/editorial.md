## The bug

The only line of the function uses the wrong operator:

```python
return sum(grades) // len(grades)
```

`//` is floor division: it rounds the quotient down to a whole number. That is only correct when the sum happens to be a multiple of the count, as in Example 3 (88 / 4 = 22). With `grades = [1, 2]` the code returns `1` instead of `1.5`, and in Example 1 it returns `82` instead of `82.333...`.

## The fix

Use true division, which keeps the fractional part:

```python
def average_grade(grades: list[int]) -> float:
    return sum(grades) / len(grades)
```

In TypeScript, return `total / grades.length` without the `Math.floor` call.

**Complexity:** O(n) time, O(1) extra space.
