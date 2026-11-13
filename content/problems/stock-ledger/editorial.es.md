## Enfoque: un árbol de Fenwick

Un árbol de Fenwick sobre `n` posiciones guarda en `tree[k]` la suma de un bloque de posiciones que termina en `k` y cuya longitud es el bit menos significativo de `k`. Dos bucles cortos lo resuelven todo:

- **sumar(i, d):** para `k = i + 1`, luego `k += k & −k`, mientras `k ≤ n`: `tree[k] += d`.
- **prefijo(i)** (posiciones 0 a i − 1): para `k = i`, luego `k −= k & −k`, mientras `k > 0`: suma `tree[k]`.

Constrúyelo sumando cada `stock[i]` inicial. Un `"set"` se convierte en `sumar(i, count − actual[i])` (y se actualiza `actual[i]`); un `"total"` es `prefijo(b + 1) − prefijo(a)`.

**Complejidad:** O((n + q) log n) en tiempo y O(n) de espacio. Sumar los estantes en cada `"total"` cuesta O(n) por consulta, O(n·q) en total.
