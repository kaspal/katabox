## Enfoque: búsqueda binaria sobre la respuesta

Sea `need(s)` las horas a velocidad `s`: la suma de ⌈p / s⌉ sobre todos los trabajos. Nunca aumenta al crecer `s`, y `need(max(pages)) = pages.length ≤ hours`. Así que las velocidades que funcionan son exactamente `respuesta, respuesta + 1, …`, y podemos buscar el límite con búsqueda binaria:

- `lo = 1`, `hi = max(pages)`.
- Mientras `lo < hi`: sea `mid = (lo + hi) / 2`. Si `need(mid) ≤ hours`, la respuesta es como mucho `mid` (`hi = mid`); si no, está por encima (`lo = mid + 1`).

Calcula ⌈p / s⌉ con enteros como `(p + s - 1) / s`. El total puede llegar a 5·10¹³, así que usa enteros de 64 bits.

### Paso a paso

`pages = [7, 3, 12]`, `hours = 6`:

| lo | hi | mid | need(mid) | movimiento |
| --- | --- | --- | --- | --- |
| 1 | 12 | 6 | 2 + 1 + 2 = 5 | cabe: hi = 6 |
| 1 | 6 | 3 | 3 + 1 + 4 = 8 | muy lenta: lo = 4 |
| 4 | 6 | 5 | 2 + 1 + 3 = 6 | cabe: hi = 5 |
| 4 | 5 | 4 | 2 + 1 + 3 = 6 | cabe: hi = 4 |

`lo = hi = 4`.

**Complejidad:** O(n log M) en tiempo con n trabajos y M = max(pages), y O(1) de espacio extra. Probar las velocidades 1, 2, 3, … cuesta O(n·M) cuando la respuesta es grande.
