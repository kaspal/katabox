## Enfoque: búsqueda binaria sobre la pendiente

Compara un hito `m` con el siguiente:

- `heights[m] < heights[m + 1]`: la ruta sube en `m`, así que la cumbre está después de `m`;
- si no, la ruta baja desde `m` (o `m` es la cumbre), así que la cumbre está en `m` o antes.

Mantén la cumbre dentro de `[lo, hi]`, empezando por toda la ruta. Mientras `lo < hi`, mira `m = ⌊(lo + hi) / 2⌋` (siempre `< hi`, así que `m + 1` existe) y quédate con la mitad que contiene la cumbre. Cuando `lo = hi`, esa es la cumbre.

### Paso a paso

`[1, 4, 9, 7, 2]`: lo = 0, hi = 4, m = 2: 9 > 7, así que hi = 2. m = 1: 4 < 9, así que lo = 2. La cumbre es la 2.

**Complejidad:** O(log n) en tiempo y O(1) de espacio.
