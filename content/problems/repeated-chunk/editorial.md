## Approach: binary search on the length, rolling hashes to check

**Monotonic.** A repeated piece of length L contains a repeated piece of every shorter length, so `repeats(L)` is true up to the answer and false after it. Binary search for the largest L in `[0, n − 1]` with `repeats(L)`.

**Checking one length.** Treat letters as digits in base `B` and compute prefix hashes `h[i]` of `text[0..i)` modulo a prime `M`, with powers `pw[L] = B^L mod M`. The hash of `text[i..i+L)` is `(h[i+L] − h[i] · pw[L]) mod M`. Put all n − L + 1 hashes in a set; a hash seen twice means the piece repeats.

Different pieces can share a hash by accident. Using two independent moduli (and comparing the pair) makes that so unlikely that it does not happen on these tests. The moduli follow the batch's rule for JavaScript: both are primes below 2²⁶ (50 331 653 and 25 165 843), so every product stays below 2⁵³ and is exact. Python's reference uses one modulus, 2⁶¹ − 1, since its integers are unbounded.

### Walkthrough

"mississippi": L = 5 has no repeat, L = 4 does ("issi" at 1 and 4), so the answer is 4.

**Complexity:** O(n log n) expected time, O(n) space. Comparing every pair of starting positions letter by letter is O(n²) pairs at least.
