## Approach: merge the congruences one by one (generalized Chinese remainder theorem)

Line `(r, m)` leaves at minute `t` exactly when `t ≡ r (mod m)` (with `0 ≤ r < m`, every `t ≥ 0` with that remainder is one of its departures). So a timetable asks for the smallest `t ≥ 0` that satisfies all its congruences at once.

**Why searching is too slow.** The pattern repeats after the lcm of the periods, so checking every minute below it is correct, but the lcm can reach 9·10¹⁵. A smarter search walks only the departures of the line with the longest period, `r, r + m, r + 2m, …`; that still takes up to lcm / m steps, and with periods of at most 10⁶ and an lcm of 10¹² or more that is at least a million steps, often billions, for a single timetable. Instead, fold the congruences into a single one.

**Merging two congruences.** Suppose every line so far leaves at `t ≡ x (mod M)`, with `0 ≤ x < M`. Those minutes are `t = x + M·k`. The next line needs

```
x + M·k ≡ r (mod m)   ⇔   M·k ≡ r − x (mod m)
```

Let `g = gcd(M, m)`. The left side is a multiple of `g`, and so is `m`, so a solution needs `g | (r − x)`; if not, the lines never meet: print `-1`. Otherwise divide everything by `g`: with `m' = m / g`, `M' = M / g` and `d = (r − x) / g`,

```
M'·k ≡ d (mod m'),   gcd(M', m') = 1   ⇒   k ≡ d · inverse(M', m') (mod m')
```

Take that `k` in `[0, m')`. Then `x ← x + M·k` and `M ← M·m'`, which is `lcm(M, m)`. Start from `(x, M) = (0, 1)` (every minute) and merge each line in turn; the final `x` is the answer.

**The inverse** comes from the extended Euclidean algorithm: running Euclid on `(a, m')` while tracking the coefficient `s` with `a·s ≡ remainder (mod m')` ends with remainder 1 and `a·s ≡ 1`. When `m' = 1` the inverse is 0 and `k = 0`.

**Why the result is the smallest minute:** the solutions of the merged system are exactly `x (mod lcm)`, and after each step `0 ≤ x ≤ (M − 1) + M·(m' − 1) = M·m' − 1`, so `x` is the smallest non-negative one.

**Staying exact.** Every value fits in a 64-bit integer, and even in a TypeScript number, which is exact up to 2⁵³ ≈ 9.007·10¹⁵:

- `M`, `x` and every partial lcm are at most the final lcm ≤ 9·10¹⁵; `r − x`, the gcd and the exact division `(r − x) / g` stay within that size too;
- `d mod m'` and the inverse are both below `m' ≤ 10⁶`, so their product is below 10¹²;
- `M·k < M·m' = lcm ≤ 9·10¹⁵`, and so is the new `x`.

So no 128-bit arithmetic or modular-multiplication trick is needed, as long as you multiply only these pairs (never `M` by an inverse, for example). Reduce `d` into `[0, m')` first, since `r − x` may be negative (`((d % m') + m') % m'` in TypeScript and PHP, `rem_euclid` in Rust; Python's `%` is already non-negative).

**Complexity:** O(n·log m) per timetable, O(T·n·log m) in all.

**Input:** read all tokens; each timetable is `n` followed by `2n` numbers. Even after a `-1` is certain, keep reading that timetable's remaining lines.
