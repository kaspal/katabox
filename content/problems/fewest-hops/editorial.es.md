## Enfoque: búsqueda en anchura

Como cada salto cuesta 1, los saltos mínimos hasta cada estación se encuentran explorando la red por anillos alrededor de la estación 0: la búsqueda en anchura (BFS).

```
dist = [-1] * n; dist[0] = 0
cola = [0]
para v en cola:                    # las estaciones salen por orden de distancia
    para u en vecinos[v]:
        si dist[u] == -1:
            dist[u] = dist[v] + 1
            cola.añadir(u)
devolver dist[n − 1]
```

Una estación se alcanza por primera vez por una ruta mínima, porque la cola entrega las estaciones con distancia no decreciente: todas las de distancia `d` se procesan antes que cualquiera de `d + 1`. Una estación a la que nunca se llega conserva `-1`, que es justo la respuesta para "no hay ruta". Los enlaces repetidos solo añaden un vecino repetido, que la comprobación de `dist` se salta.

### Paso a paso

Ejemplo 1: desde 0 la cola recibe 1 y 3 (distancia 1); desde 1 recibe 2 y desde 3 recibe 4 (distancia 2). La estación 4 se alcanza con 2 saltos.

**Complejidad:** O(n + m) de tiempo y memoria para `m` enlaces. Relajar todos los enlaces una y otra vez hasta que nada mejore (como hace Bellman–Ford) cuesta O(n · m).
