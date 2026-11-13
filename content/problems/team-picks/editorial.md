## Approach: factorial tables and modular inverses

Let N be the largest `n` asked. Since N ≤ 10⁵ < p = 1 000 003, none of 1..N is a multiple of p, so all the factorials have inverses modulo p.

1. `fact[0] = 1`, `fact[i] = fact[i − 1] · i mod p`.
2. `inv[N] = fact[N]^(p − 2) mod p` (by Fermat's little theorem, `x^(p−1) ≡ 1`, so `x^(p−2)` is the inverse of `x`), computed by squaring.
3. Going down, `inv[i − 1] = inv[i] · i mod p`, because `1/(i−1)! = i / i!`.
4. A query `[n, k]` is `0` if `k > n`, otherwise `fact[n] · inv[k] mod p · inv[n − k] mod p`.

Every product has both factors below p, so it stays below about 10¹², exact in every language here.

**Complexity:** O(N + log p) to prepare, O(1) per query. Multiplying k factors per query is O(q · k), up to 10⁹ steps.
