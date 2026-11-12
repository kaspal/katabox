## Enfoque: algoritmo de Dijkstra con un montículo

Construye listas de adyacencia (cada carretera en los dos sentidos). Guarda `dist[t]`, el coste más barato conocido hasta cada pueblo, empezando en ∞ salvo `dist[start] = 0`, y un montículo de mínimos con `(0, start)`.

Saca una y otra vez el más barato `(d, t)`:

- si `d > dist[t]`, es una entrada desactualizada: ignórala;
- si `t == goal`, devuelve `d`;
- si no, para cada carretera `(t, u, toll)`, si `d + toll < dist[u]`, pon `dist[u] = d + toll` y métela en el montículo.

Si el montículo se vacía antes, `goal` es inalcanzable: devuelve `-1`.

Como los peajes son positivos, la primera vez que se saca un pueblo su coste ya es definitivo.

### Paso a paso

Ejemplo 1, desde el pueblo 0:

| saca | mejora | montículo después |
| --- | --- | --- |
| (0, 0) | 1 → 5, 2 → 2 | (2, 2), (5, 1) |
| (2, 2) | 3 → 8, 1 → 3 | (3, 1), (5, 1), (8, 3) |
| (3, 1) | 3: 3 + 5 = 8, no es más barato | (5, 1), (8, 3) |
| (5, 1) | desactualizada | (8, 3) |
| (8, 3) | destino | |

**Complejidad:** O((n + m) log m) en tiempo y O(n + m) en espacio. Bellman-Ford (relajar todas las carreteras, ronda tras ronda, hasta que nada cambie) cuesta O(n·m): hasta 10⁹ pasos cuando la ruta más barata tiene muchas carreteras.
