## The idea

In ordinary arithmetic, dividing by `a` means multiplying by `1 / a`. Modulo `m` there are no fractions, but there may be a whole number that plays the same role: the **modular inverse** of `a` is the `x` in `[0, m)` with `a · x mod m = 1`. Once you have it, "divide by `a`" becomes "multiply by `x`".

It exists exactly when `gcd(a, m) = 1`. The reason is **Bézout's identity**: Euclid's divisions can be run backwards to write the gcd as a combination `a · x + m · y = gcd(a, m)` with whole numbers `x` and `y`. If the gcd is 1, reading that equation mod `m` drops the `m · y` term and leaves `a · x ≡ 1`. If the gcd is some `g > 1`, every `a · x - m · y` is a multiple of `g`, so it can never be 1, and there is no inverse.

The **extended Euclidean algorithm** finds `x` while doing the ordinary divisions. Each remainder `r` carries a coefficient `x` with `r ≡ a · x (mod m)`. The pair starts as `a mod m` (coefficient 1) and `m` (coefficient 0, since `m ≡ 0`). Every division with quotient `q` makes a new remainder `old_r - q · r`, and the same combination of coefficients, `old_x - q · x`, keeps the rule true. The `y` side is never needed, so the sample does not track it. When the remainder 0 appears, `old_r` is the gcd: if it is 1, `old_x` is the inverse, shifted into `[0, m)`; otherwise the output is `[-1]`. The number and the modulus are typed as two fields.

### The default input

`a = 17`, `m = 101` (the "a prime modulus" preset). As `(remainder, coefficient)` pairs: `(17, 1)` and `(101, 0)`. The first quotient is 0, which only swaps them. Then `101 = 5 · 17 + 16` gives `(16, 0 - 5 · 1) = (16, -5)`; `17 = 1 · 16 + 1` gives `(1, 1 - 1 · (-5)) = (1, 6)`; and `16 = 16 · 1 + 0` ends the loop. Four divisions, gcd 1, coefficient 6. The output is `[6]`: `17 · 6 = 102 = 101 + 1`.

## Steps

1. Start with `(old_r, r) = (a mod m, m)` and `(old_x, x) = (1, 0)`.
2. While `r ≠ 0`: `q = old_r div r`; replace `(old_r, r)` by `(r, old_r - q · r)` and `(old_x, x)` by `(x, old_x - q · x)`.
3. If `old_r ≠ 1`, return `-1`; else return `old_x mod m`, as a value in `[0, m)`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(1)`: when `a mod m` is 0 or 1, a couple of divisions settle it |
| Average and worst | `O(log m)`: the same divisions as Euclid's algorithm |
| Extra space | `O(1)` |

## Pitfalls

- **A negative result.** The coefficient is often negative; in languages where `%` keeps the sign, take the remainder, add `m` and take it again.
- **Assuming an inverse exists.** The "shares a factor" preset, 12 and 30, ends with gcd 6 and returns `[-1]`.
- **Fermat only for primes.** When `m` is prime, `a^(m - 2) mod m` is also the inverse (Fermat's little theorem), computed with [Fast modular power](learn:math-geometry/fast-power). For a modulus that is not prime, it generally gives a wrong answer; the extended algorithm works for any `m`.

## When to use it

Use it to divide under a modulus: averages and probabilities "modulo a prime", undoing a multiplication in a hash, or computing `C(n, k) = n! / (k! · (n - k)!)` modulo a prime from factorial tables. It also solves `a · x ≡ b (mod m)`: multiply `b` by the inverse.

## Related topics

- [Euclid's GCD](learn:math-geometry/euclid-gcd): the same divisions, without the coefficient.
- [Fast modular power](learn:math-geometry/fast-power): the Fermat route for a prime modulus.
- [Pascal's triangle and C(n, k)](learn:math-geometry/pascal-ncr): binomial coefficients without any division.
