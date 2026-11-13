## Enfoque: tablas de factoriales e inversos modulares

Sea N el mayor `n` consultado. Como N ≤ 10⁵ < p = 1 000 003, ninguno de 1..N es múltiplo de p, así que todos los factoriales tienen inverso módulo p.

1. `fact[0] = 1`, `fact[i] = fact[i − 1] · i mod p`.
2. `inv[N] = fact[N]^(p − 2) mod p` (por el pequeño teorema de Fermat, `x^(p−1) ≡ 1`, así que `x^(p−2)` es el inverso de `x`), calculado por cuadrados.
3. Hacia abajo, `inv[i − 1] = inv[i] · i mod p`, porque `1/(i−1)! = i / i!`.
4. Una consulta `[n, k]` vale `0` si `k > n`, y si no, `fact[n] · inv[k] mod p · inv[n − k] mod p`.

Cada producto tiene los dos factores por debajo de p, así que queda por debajo de unos 10¹², exacto en todos los lenguajes de aquí.

**Complejidad:** O(N + log p) para preparar y O(1) por consulta. Multiplicar k factores por consulta es O(q · k), hasta 10⁹ pasos.
