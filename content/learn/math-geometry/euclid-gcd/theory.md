## The idea

The **greatest common divisor** of two positive whole numbers, `gcd(a, b)`, is the largest number that divides both. Trying every candidate from `min(a, b)` downwards works, but it can take hundreds of tries. Euclid's algorithm needs only a handful of divisions.

It rests on one fact: **`a` and `b` have exactly the same common divisors as `b` and `a mod b`**. Write `a = q · b + r`. Anything that divides `a` and `b` also divides `r = a - q · b`; anything that divides `b` and `r` also divides `a = q · b + r`. So replacing the pair `(a, b)` by `(b, a mod b)` never changes the answer, and the numbers keep shrinking. When `b` reaches 0, the pair is `(g, 0)`, and the largest divisor of `g` and 0 is `g` itself.

The sample repeats that one replacement while `b` is not 0 and counts the divisions. The output is `[gcd, number of divisions]`. If `a` is smaller than `b`, the first division just swaps them, since `a mod b = a`.

Once the gcd is known, the **least common multiple** follows: `lcm(a, b) = a · b / gcd(a, b)`. Divide before multiplying, `a / gcd · b`, so the intermediate value stays small.

### The default input

`a = 336`, `b = 252` (the "a big common factor" preset). First division: `336 mod 252 = 84`, so the pair becomes `(252, 84)`. Second: `252 mod 84 = 0`, so it becomes `(84, 0)` and the loop stops. The output is `[84, 2]`: two divisions. Both numbers are multiples of 84 (`336 = 4 · 84`, `252 = 3 · 84`), and their lcm is `336 / 84 · 252 = 1008`.

## Steps

1. While `b` is not 0: replace `(a, b)` by `(b, a mod b)` and count one division.
2. Return `a` and the count.

## Cost

| Case | Cost |
|---|---|
| Best | `O(1)`: one division when `b` divides `a` |
| Average and worst | `O(log min(a, b))` divisions |
| Extra space | `O(1)`: two numbers and a counter |

Why so fast? After two divisions, the first number of the pair is `a mod b`, which is less than half of `a` (take `a ≥ b`): if `b ≤ a / 2`, then `a mod b < b ≤ a / 2`; otherwise `a mod b = a - b < a / 2`. Halving every two steps gives a logarithmic count. The slowest pairs are **neighbouring Fibonacci numbers**, where almost every quotient is 1 and the numbers shrink as little as possible: the "neighbouring Fibonacci numbers" preset, 610 and 377, needs 13 divisions to reach `[1, 13]`.

## Pitfalls

- **Subtracting instead of dividing.** The oldest form subtracts `b` from `a` again and again; with `a = 999` and `b = 1` that is 999 steps. The remainder does all of them at once.
- **Overflow in the lcm.** `a · b` can exceed the integer range even when the lcm fits; divide by the gcd first.
- **Zero inputs.** `gcd(a, 0) = a`, and the loop already handles it. `gcd(0, 0)` comes out as 0, the usual convention, even though every number divides 0.

## When to use it

Use it to simplify fractions, to find the largest tile or step that fits two lengths exactly, or as a building block for lcm and modular inverses. The linked problem, Shared Tile, asks for the largest square tile that covers a floor exactly: that is the gcd of the two sides.

## Related topics

- [Modular inverse](learn:math-geometry/modular-inverse): the same divisions, also tracking a coefficient.
- [Sieve of Eratosthenes](learn:math-geometry/sieve-eratosthenes): divisibility from the prime side.
- [Big-O growth curves](learn:foundations/big-o-growth): what a logarithmic number of steps means in practice.
