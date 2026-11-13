## Enfoque: caminos mínimos entre todas las parejas (Floyd–Warshall)

Con como mucho 100 puertos y hasta 10⁴ consultas, calcula una vez el tiempo mínimo entre **todas** las parejas:

1. `dist[i][i] = 0`; `dist[a][b] = dist[b][a] = min(actual, minutes)` para cada ruta (quédate con la más rápida de las rutas paralelas); todo lo demás ∞.
2. Para `k = 0 … n − 1`, para todo `i, j`: `dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])`.

Tras el paso 2 con un `k` dado, `dist[i][j]` es el tiempo mínimo usando como escalas solo los puertos `0..k`; tras el último `k`, cualquier puerto. Cada consulta es entonces una búsqueda directa, con ∞ devuelto como −1.

**Complejidad:** O(n³ + m + q) en tiempo y O(n²) de espacio: como mucho 10⁶ actualizaciones. Lanzar Dijkstra para cada consulta también es correcto, pero repite trabajo entre consultas.
