Hay muchas consultas pero pocos puertos. ¿Podrías calcular la respuesta para **todas** las parejas de puertos una vez y después solo consultarla?
---
Floyd–Warshall: `dist[i][j]` empieza como la mejor ruta directa (0 en la diagonal, ∞ si no hay). Después permite los puertos 0, 1, …, n − 1 de uno en uno como escalas.
---
Para cada escala `k`, para todo `i, j`: `dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])`. Tras el último `k`, `dist` tiene todos los tiempos mínimos; ∞ significa inalcanzable (−1).
