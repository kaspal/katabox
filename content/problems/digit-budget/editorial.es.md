## Enfoque: DP sobre las cifras

Escribe `limit` como sus cifras `d[0..L)`, la más significativa primero. Cuenta los números de 0 a `limit` (como cadenas de L cifras con ceros a la izquierda, que no cambian la suma) cuyas cifras suman `total`:

```
count(pos, suma, ajustado):
    si suma > total: devuelve 0
    si pos == L: devuelve 1 si suma == total, si no 0
    tope = d[pos] si ajustado, si no 9
    devuelve Σ_{x = 0..tope} count(pos + 1, suma + x, ajustado y x == tope)
```

Los estados con `ajustado = false` solo dependen de `(pos, suma)` y se repiten constantemente, así que se memorizan: como mucho 16 × 136 estados con 10 opciones cada uno. El camino ajustado es una sola cadena. Como `total ≥ 1`, el número 0 (suma 0) nunca se cuenta, así que la respuesta son exactamente los boletos 1..limit.

**Complejidad:** O(L · total · 10) en tiempo, unos 2·10⁴ pasos. Comprobar cada número hasta `limit` es O(limit · L).
