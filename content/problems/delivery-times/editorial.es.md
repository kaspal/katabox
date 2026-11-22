## El error

El código inicial usa un montículo, pero anota las distancias demasiado pronto:

```python
dist[start] = 0
heap = [(0, start)]
while heap:
    d, v = heapq.heappop(heap)
    for u, w in adj[v]:
        if dist[u] == -1:
            dist[u] = d + w
            heapq.heappush(heap, (d + w, u))
```

Un punto recibe su distancia la primera vez que lo alcanza cualquier vecino. Esa primera calle no tiene por qué formar parte de la ruta más corta: un camino más barato que pase por un punto extraído más tarde nunca se tiene en cuenta, porque para entonces `dist[u]` ya no es `-1`. El orden del montículo es correcto, pero la respuesta se escribe antes de que el montículo pueda decidir.

Con `n = 3`, `roads = [[0, 1, 5], [0, 2, 1], [2, 1, 1]]` y `start = 0`: al extraer el punto 0 se asigna `dist[1] = 5` y `dist[2] = 1`. Al extraer el punto 2, el punto 1 ya tiene valor, así que el código devuelve `[0, 5, 1]`. La ruta 0 → 2 → 1 cuesta 2 minutos, así que la respuesta es `[0, 2, 1]`.

## La corrección

Una distancia solo es definitiva cuando su punto sale del montículo, porque ya no puede quedar nada más corto esperando. Quita la línea `dist[start] = 0` y cambia el bucle por:

```python
while heap:
    d, v = heapq.heappop(heap)
    if dist[v] != -1:
        continue
    dist[v] = d
    for u, w in adj[v]:
        if dist[u] == -1:
            heapq.heappush(heap, (d + w, u))
```

La versión en TypeScript necesita el mismo cambio: quita `dist[start] = 0` y, después de `heap.pop()`, salta los puntos cuyo `dist` ya está asignado, asigna `dist[v] = d` y mete los vecinos sin tocar su `dist`.

**Complejidad:** O((n + m) log m) en tiempo para m calles, y O(n + m) de espacio.
