C(n, k) = n! / (k! · (n − k)!). Computing it from scratch for every query is O(k) per query. What can you prepare once for all queries?
---
Precompute the factorials 0!, 1!, …, max n! modulo p. Division is the problem: modulo a prime p, dividing by `x` is multiplying by its **inverse**, `x^(p−2) mod p` (Fermat's little theorem).
---
Precompute the inverse factorials too: compute `inv[N] = (N!)^(p−2)` with fast power, then `inv[i − 1] = inv[i] · i` going down. Each query is then `fact[n] · inv[k] · inv[n − k] mod p`.
