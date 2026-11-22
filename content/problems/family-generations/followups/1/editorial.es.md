## Idea

Calcula todas las profundidades con un recorrido en anchura desde la fundadora (los números no siguen las generaciones, así que construye primero las listas de hijos). La respuesta para `[a, b]` es `prof[a] + prof[b] − 2·prof[c]`, donde `c` es su antepasado común más bajo. Encuentra `c` con saltos binarios: `up[0][v]` es el progenitor (la fundadora apunta a sí misma) y `up[k][v] = up[k−1][up[k−1][v]]`. Primero sube a la persona más profunda la diferencia de profundidades, un salto de la tabla por cada bit activo. Si ahora son la misma persona, esa es `c`. Si no, para cada `k` de mayor a menor, haz saltar a las dos `2ᵏ` generaciones siempre que los destinos sean distintos: se quedan justo por debajo del punto de encuentro, así que `c` está un nivel por encima de cualquiera de ellas.

## Complejidad

O(n log n) en tiempo y memoria para construir la tabla, y después O(log n) por pareja: O((n + q) log n) en tiempo en total.
