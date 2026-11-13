## Enfoque: dos punteros con máximos acumulados

El agua encima de la columna `k` es `min(L[k], R[k]) − columns[k]`, donde `L[k]` es la columna más alta de `0..k` y `R[k]` la más alta de `k..n−1` (las dos incluyen `k`, así que el resultado nunca es negativo).

Dos arrays de máximos prefijos dan una solución O(n) en tiempo y O(n) en espacio. Dos punteros ahorran los arrays:

- `i = 0`, `j = n − 1`, `leftMax = rightMax = 0`;
- mientras `i ≤ j`: actualiza `leftMax` con `columns[i]` y `rightMax` con `columns[j]`;
  - si `leftMax ≤ rightMax`: a la derecha de la columna `i` hay una columna de al menos `rightMax ≥ leftMax`, así que `min(L[i], R[i]) = leftMax`; suma `leftMax − columns[i]` y haz `i++`;
  - si no, de forma simétrica, suma `rightMax − columns[j]` y haz `j--`.

### Paso a paso

`[3, 0, 2, 0, 4]`: leftMax pasa a 3 y se queda por debajo de rightMax = 4, así que el puntero izquierdo recorre todo: 3 − 3, 3 − 0, 3 − 2, 3 − 0 y por último el propio 4 suma 0. Total 7.

**Complejidad:** O(n) en tiempo y O(1) de espacio. Recorrer los dos lados para cada columna es O(n²), 4·10¹⁰ pasos con 2·10⁵ columnas.
