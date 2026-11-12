## Approach: Euclid's algorithm

A square tile of side `s` fills the floor with no cuts exactly when `s` divides both sides. The largest such `s` is the greatest common divisor of `width` and `height`.

Euclid's algorithm repeatedly replaces the pair `(a, b)` with `(b, a mod b)`. Any number dividing `a` and `b` also divides `a mod b = a − q·b`, and the other way round, so the common divisors never change. When `b` reaches 0, `a` is the answer.

```
while height != 0:
    width, height = height, width % height
return width
```

### Walkthrough

`(240, 180) → (180, 60) → (60, 0)`: the answer is 60.

**Complexity:** O(log min(width, height)) steps: the numbers shrink at least as fast as the Fibonacci numbers grow. Trying every size from the smaller side down is O(min(width, height)), around 10¹² steps for the largest floors.
