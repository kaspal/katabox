## Enfoque: un árbol de Fenwick sobre las diferencias

Sea `diff` un arreglo que empieza a ceros. Sumar `delta` a los medidores `a..b` cambia las diferencias en dos sitios: `diff[a] += delta` y `diff[b + 1] −= delta` (si `b + 1` es un medidor). La corrección del medidor `i` es entonces `diff[0] + … + diff[i]`, una suma prefija.

Un árbol de Fenwick guarda `diff` de modo que tanto una actualización puntual como una suma prefija cuestan O(log n):

- `"add" [a, b, delta]`: `sumar(a, delta)`, y `sumar(b + 1, −delta)` cuando `b + 1 < n`.
- `"read" [i]`: `readings[i] + prefijo(i + 1)`.

### Paso a paso

`readings = [10, 20, 30, 40]`: `"add" [1, 3, 5]` pone +5 en 1. `"read" [2]` es 30 + (0 + 5 + 0) = 35. `"add" [0, 1, −2]` pone −2 en 0 y +2 en 2. `"read" [1]` es 20 + (−2 + 5) = 23.

**Complejidad:** O(q log n) en tiempo y O(n) de espacio. Actualizar cada medidor del rango cuesta O(n) por `"add"`.
