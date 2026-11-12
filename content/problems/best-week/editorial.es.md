## Enfoque: deslizar una ventana fija

Dos tramos vecinos comparten `k − 1` días, así que no hace falta volver a sumarlos.

1. `window` = la suma de los primeros `k` días; `best = window`.
2. Para `i = k … n − 1`: `window += sales[i] − sales[i − k]` y `best = max(best, window)`.

### Paso a paso

`sales = [12, 30, 7, 25, 18, 4]`, `k = 3`:

| tramo (días) | cambio | total | mejor |
| --- | --- | --- | --- |
| 0-2 | primera suma | 49 | 49 |
| 1-3 | +25 −12 | 62 | 62 |
| 2-4 | +18 −30 | 50 | 62 |
| 3-5 | +4 −7 | 47 | 62 |

**Complejidad:** O(n) en tiempo y O(1) de espacio extra. Volver a sumar cada tramo cuesta O(n·k), hasta 10¹⁰ sumas para 2·10⁵ días.
