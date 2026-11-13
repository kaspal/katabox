## Enfoque: DFS con valores low-link (Tarjan)

Lanza un DFS numerando las islas en orden de visita: `tin[v]`. Define `low[v]` como el menor `tin` al que se llega desde el subárbol del DFS de `v` usando pasarelas del árbol hacia abajo y como mucho una pasarela que no es del árbol (una "arista de retroceso") hacia arriba.

Cuando el DFS vuelve del hijo `c` al padre `p` por la pasarela `e`:

- `low[p] = min(low[p], low[c])`;
- si `low[c] > tin[p]`, ninguna isla bajo `c` puede volver a `p` o más arriba sin `e`: **`e` es crítica**.

Una pasarela que no es del árbol `v–u` hacia una isla ya visitada actualiza `low[v] = min(low[v], tin[u])`. Solo se salta la pasarela usada para llegar a `v`, por su índice.

Un DFS recursivo se desbordaría con una cadena de 10⁵ islas, así que el DFS lleva una pila explícita de `(isla, índice del siguiente vecino, pasarela usada)`. Escribe cada pasarela encontrada como `[min, max]` y ordena.

**Complejidad:** O(n + m) en tiempo y espacio. Cerrar cada pasarela y volver a comprobar la conectividad es O(m·(n + m)).
