## The idea

A **prime** is a whole number above 1 whose only divisors are 1 and itself. To list every prime up to `n`, you could test each number by trial division. The sieve of Eratosthenes turns the question around: instead of asking "does anything divide `k`?", each prime **crosses out its own multiples**, and whatever is never crossed out is prime.

Write the numbers from 0 to `n` (the app shows them ten to a row) and cross out 0 and 1. Take `p = 2`: it is not crossed out, so it is prime, and its multiples are not. Move on to 3, 4, 5, … and repeat for every `p` that is still unmarked when you reach it. A number that survives to its turn was not crossed out by any smaller prime, so nothing smaller divides it: it is prime.

Two shortcuts make it fast:

- **Start at `p²`.** A smaller multiple `j · p` with `j < p` has the smaller factor `j`, so the prime factors of `j` already crossed it out.
- **Stop when `p² > n`.** A composite number `k ≤ n` has a factor no larger than `√k`, so it has already fallen to that factor's prime. Every number still standing after that is prime.

The output is the number of primes, then the primes in order. The input is a single `n` from 2 to 120.

### The default input

`n = 60`. The primes 2, 3, 5 and 7 do the crossing; at `p = 8`, `8² = 64 > 60`, so the loop stops. 2 crosses 4, 6, …, 60 (29 crossings), 3 starts at 9 (18 crossings), 5 starts at 25 (8 crossings), and 7 crosses only 49 and 56. That makes 57 crossings, but only 42 different numbers: 12, for example, is hit by both 2 and 3. 49 is the only number left for 7 alone.

The output is `[17, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59]`: seventeen primes up to 60.

## Steps

1. Mark every number from 2 to `n` as possibly prime; 0 and 1 are not.
2. For `p = 2, 3, …` while `p · p ≤ n`: if `p` is still marked, cross out `p²`, `p² + p`, `p² + 2p`, … up to `n`.
3. Collect the numbers still marked; return how many there are, then the numbers.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n log log n)`: the crossings of all primes together |
| Extra space | `O(n)`: one mark per number |

Prime `p` crosses at most `n / p` numbers. Adding `n / 2 + n / 3 + n / 5 + …` over the primes up to `√n` grows like `n log log n`, which is almost linear: `log log n` is below 4 for any `n` a computer can store.

## Pitfalls

- **Starting at `2p`.** It still works, but every crossing below `p²` is wasted work.
- **Comparing with a floating-point square root.** Rounding can skip the last prime that should cross; compare `p · p ≤ n` in integers.
- **Forgetting 0 and 1.** Neither is prime; leave them marked and the count is off by two.
- **A huge `n`.** One mark per number limits the sieve to what fits in memory; for a single number, trial division up to `√k` is enough.

## When to use it

Use it when you need many primes, or many "is this prime?" answers, up to a fixed limit: sieve once, then every answer is a lookup. The linked problem, Prime Lockers, asks how many primes there are up to each of many limits: one sieve plus a running count answers them all.

## Related topics

- [Euclid's GCD](learn:math-geometry/euclid-gcd): common divisors of two numbers without factoring them.
- [Prefix sums](learn:foundations/prefix-sums): turn the sieve's marks into "how many primes up to `L`" in `O(1)` per question.
- [Big-O growth curves](learn:foundations/big-o-growth): how close `n log log n` stays to `n`.
