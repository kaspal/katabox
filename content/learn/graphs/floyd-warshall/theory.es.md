## La idea

**Floyd-Warshall** responde a la vez a todas las preguntas de caminos más cortos: la ruta más barata desde cada nodo hasta cada uno de los demás. Trabaja sobre una **matriz de distancias**: la celda `dist[i][j]` empieza con el peso de la arista de `i` a `j`, `0` en la diagonal y `999` (que significa "no se conoce camino") en el resto.

El algoritmo es **programación dinámica** sobre los nodos por los que puede pasar una ruta. Antes de la ronda `k`, cada celda guarda la mejor ruta cuyas paradas intermedias son todas nodos de `0 .. k-1`. La ronda `k` permite además el nodo `k` como parada, y solo hay una pregunta nueva: ¿ir por `i → k → j` es más barato que la mejor ruta sin `k`? Así que para cada par prueba `dist[i][k] + dist[k][j] < dist[i][j]` y se queda con el valor menor. Tras la última ronda se permiten todos los nodos, y la matriz tiene las distancias más cortas reales.

Por eso `k` tiene que ser el bucle **exterior**. Las dos mitades `i → k` y `k → j` deben ser ya óptimas para paradas menores que `k` cuando se combinan. Con `k` dentro de los bucles de `i` y `j`, una celda quedaría cerrada antes de que estuvieran listas las rutas de las que depende.

### La entrada por defecto

Cinco nodos, dirigido: `0 1 4, 1 2 3, 0 2 9, 2 3 1, 3 0 2, 1 4 7, 3 4 2`. En la ronda 0, el nodo 3 aprende rutas que pasan por 0: `3 → 1` cuesta 2 + 4 = 6 y `3 → 2` cuesta 11. La ronda 1 baja `0 → 2` de 9 a 4 + 3 = 7, da a `0 → 4` el valor 11 y mejora `3 → 2` a 9. La ronda 2 da a `0 → 3` un 8 y a `1 → 3` un 4. La ronda 3 es la más movida, con seis mejoras; por ejemplo, `0 → 4` baja a 10 y el nodo 2 ya puede llegar a 0 (3), a 1 (7) y a 4 (3). La ronda 4 no cambia nada: el nodo 4 no tiene aristas de salida, así que ninguna ruta puede pasar por él.

La salida es la matriz entera, fila a fila: `[0, 4, 7, 8, 10, 6, 0, 3, 4, 6, 3, 7, 0, 1, 3, 2, 6, 9, 0, 2, 999, 999, 999, 999, 0]`. La última fila sigue en `999` salvo su diagonal: del nodo 4 no sale nada.

## Pasos

1. Rellena la matriz: `0` en la diagonal, el peso de cada arista en su celda y `999` en el resto.
2. Para cada `k` de `0` a `n - 1`:
3. Para cada par `i, j`: si `dist[i][k] + dist[k][j]` es menor que `dist[i][j]`, sustitúyelo.
4. Devuelve la matriz; una celda que sigue en `999` no tiene camino.

## Coste

| Operación | Coste |
|---|---|
| Ejecución completa | `O(n³)`: tres bucles anidados sobre los nodos |
| Espacio extra | `O(n²)` para la matriz, que se actualiza en el sitio |

## Errores comunes

- **Poner `k` en el bucle equivocado.** El código sigue funcionando y devuelve números, pero algunos son demasiado grandes.
- **Un "infinito" que se desborda o se queda corto.** Sumar dos infinitos no debe dar la vuelta, y la marca debe superar cualquier camino real. El `999` del ejemplo sirve porque los pesos y el número de nodos son pequeños.
- **Ciclos negativos.** Las aristas negativas no son problema, pero si un ciclo tiene peso total negativo, alguna celda de la diagonal se vuelve negativa y las distancias dejan de tener sentido. Revisar la diagonal lo detecta.

## Cuándo usarlo

Usa Floyd-Warshall cuando el grafo es pequeño (unos cientos de nodos) y necesitas distancias entre muchos pares: tablas de rutas, preguntas como "qué pueblo está más cerca de todos" o alcanzabilidad entre todos los pares. Para un solo nodo de partida, Dijkstra o BFS salen mucho más baratos.

## Temas relacionados

- [Representaciones de grafos](learn:graphs/graph-representations): la matriz de adyacencia sobre la que trabaja el algoritmo.
- [Caminos más cortos de Dijkstra](learn:graphs/dijkstra): caminos más cortos desde un solo inicio, mucho más rápido en grafos dispersos.
- [Bellman-Ford](learn:graphs/bellman-ford): otro algoritmo de relajación, para un solo inicio y pesos negativos.
