## Enfoque: programación dinámica sobre las cantidades

Sea `best[x]` el menor número de fichas que pagan exactamente `x` (∞ si es imposible). La última ficha de un pago óptimo de `x` es algún `c`, y las fichas anteriores tienen que ser un pago óptimo de `x − c`. Así que:

- `best[0] = 0`
- `best[x] = 1 + mínimo sobre c en coins con c ≤ x de best[x − c]`

Calcula `best` desde 0 hasta `amount`; cada casilla mira cada ficha una vez.

### Paso a paso

`coins = [1, 6, 10]`, `amount = 12`:

| x | 0 | 1 | … | 5 | 6 | 7 | … | 10 | 11 | 12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| best | 0 | 1 | … | 5 | 1 | 2 | … | 1 | 2 | 2 |

`best[12] = 1 + min(best[11], best[6], best[2]) = 1 + min(2, 1, 2) = 2`.

**Complejidad:** O(amount · k) en tiempo y O(amount) en espacio, con k valores de ficha. Probar todas las cantidades de cada ficha es exponencial, y la voracidad "primero la ficha más grande" no siempre es óptima.
