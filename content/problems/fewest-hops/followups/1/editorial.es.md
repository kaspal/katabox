## Idea

El problema base necesitaba una distancia; una búsqueda en anchura las calcula todas por
el mismo precio. Construye una lista de adyacencia con los dos sentidos de cada enlace,
empieza con `hops = [-1] * n` y `hops[0] = 0`, y procesa una cola desde la estación `0`:
para cada vecino `v` de la estación actual `u` con `hops[v] = -1`, fija
`hops[v] = hops[u] + 1` y mete `v` en la cola. La cola guarda las estaciones en orden de
distancia, así que la primera vez que se ve una estación es por una ruta más corta. No
pares al llegar a la estación `n − 1`; sigue hasta vaciar la cola. Las estaciones de otras
componentes se quedan con `-1`.

## Complejidad

O(n + links.length) en tiempo y memoria.
