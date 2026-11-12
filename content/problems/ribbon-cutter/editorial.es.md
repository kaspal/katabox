## Enfoque: búsqueda binaria sobre la respuesta

Sea `count(L) = Σ ⌊r / L⌋` sobre los rollos. Alargar los trozos nunca da más trozos, así que `count` no crece con `L`, y las longitudes con `count(L) ≥ pieces` son exactamente `1 … respuesta`.

1. Si `count(1) = Σ r < pieces`, devuelve 0.
2. Si no, busca en `[1, max(ribbons)]` el último `L` con `count(L) ≥ pieces`:
   - `lo = 1` (sirve), `hi = max(ribbons)`;
   - mientras `lo < hi`: `mid = (lo + hi + 1) / 2`; si `count(mid) ≥ pieces` entonces `lo = mid`, si no `hi = mid − 1`.

### Paso a paso

`ribbons = [30, 47, 18]`, `pieces = 6`:

| lo | hi | mid | count(mid) | movimiento |
| --- | --- | --- | --- | --- |
| 1 | 47 | 24 | 1 + 1 + 0 = 2 | hi = 23 |
| 1 | 23 | 12 | 2 + 3 + 1 = 6 | lo = 12 |
| 12 | 23 | 18 | 1 + 2 + 1 = 4 | hi = 17 |
| 12 | 17 | 15 | 2 + 3 + 1 = 6 | lo = 15 |
| 15 | 17 | 16 | 1 + 2 + 1 = 4 | hi = 15 |

Respuesta 15.

**Complejidad:** O(n log M) en tiempo con M el rollo más largo, y O(1) de espacio extra. Probar longitudes una a una desde el rollo más largo hacia abajo cuesta O(n·M).
