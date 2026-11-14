## La idea

El **algoritmo de Dijkstra** encuentra la ruta más barata desde un nodo de partida hasta todos los demás cuando cada arista tiene un peso **no negativo**. Hace crecer un conjunto de nodos fijados hacia fuera, en orden de distancia: en cada paso fija el nodo sin fijar con la menor distancia conocida y después intenta mejorar a sus vecinos a través de él.

La herramienta es una **cola de prioridad** de pares `(distancia, nodo)`, primero la menor distancia y, si empatan, el nodo más pequeño. Cuando sale un par, su distancia es definitiva. Después cada arista `u → v` con peso `w` se **relaja**: si `d + w` mejora la mejor distancia de `v` hasta ahora, se apunta y se mete `(d + w, v)`. Un nodo puede estar varias veces en la cola con distancias distintas; las entradas viejas y mayores están **caducadas**, y cuando sale una simplemente se ignora (borrado perezoso). Los nodos a los que nunca se llega dan `-1`.

Python tiene `heapq`. TypeScript no trae una cola de prioridad, así que el ejemplo en TS mantiene un arreglo normal ordenado tras cada inserción, lo que basta para una docena de nodos; un programa real usaría un montículo binario.

### La entrada por defecto

Seis nodos, no dirigido: `0 1 7, 0 2 2, 2 1 3, 1 3 1, 2 4 8, 3 4 2, 4 5 3, 3 5 9`. Desde 0, el nodo 1 recibe 7 y el nodo 2 recibe 2. El nodo 2 sale primero y encuentra un rodeo: 2 + 3 = 5 mejora el 7 del nodo 1, y 4 recibe 10. El nodo 1 (5) da a 3 la distancia 6. Después sale la entrada vieja `(7, 1)` y se ignora. El nodo 3 (6) mejora 4 a 8 y da a 5 la distancia 15; el nodo 4 (8) mejora 5 a 11. Las entradas `(10, 4)` y `(15, 5)` también están caducadas.

La salida es `[0, 5, 2, 6, 8, 11]`. La forma más barata de llegar a 5 es 0 → 2 → 1 → 3 → 4 → 5: cinco aristas, y aun así más barata que cualquier ruta más corta.

## Pasos

1. Pon la distancia del nodo de partida a 0, las demás a "desconocida", y mete `(0, inicio)`.
2. Saca el par más pequeño `(d, u)`. Si `d` es mayor que la mejor distancia de `u`, ignóralo.
3. Para cada arista `u → v` con peso `w`: si `v` es desconocido o `d + w` es menor que su distancia, ponla a `d + w` y mete `(d + w, v)`.
4. Repite hasta que la cola esté vacía; las distancias desconocidas pasan a ser `-1`.

## Coste

| Operación | Coste |
|---|---|
| Ejecución completa con un montículo binario | `O((n + m) log n)`: hasta `m` inserciones y extracciones, cada una `O(log m) = O(log n)` |
| Espacio extra | `O(n + m)`: las distancias y hasta una entrada en la cola por relajación |

## Errores comunes

- **Pesos negativos.** Fijar un nodo da por hecho que nada encontrado después puede ser más barato, porque cada ruta pendiente solo suma peso. Una arista negativa rompe esa promesa: un nodo puede quedar fijado y después una ruta más larga que pasa por una arista negativa llega a él más barata. Usa Bellman-Ford en su lugar.
- **Parar la primera vez que se ve un nodo.** Solo sacarlo de la cola lo fija; una distancia puesta al relajar todavía puede mejorar.

## Cuándo usarlo

Usa Dijkstra para mapas de carreteras, enrutamiento en redes, mapas de juegos con costes de terreno y cualquier búsqueda en la que los movimientos tengan costes distintos y no negativos. Si todos cuestan lo mismo, BFS es más sencillo y más rápido.

## Temas relacionados

- [Montículo binario y cola de prioridad](learn:trees/binary-heap): la cola que hay detrás del algoritmo.
- [Búsqueda en anchura (BFS)](learn:graphs/bfs): el caso particular en el que cada arista cuesta 1.
- [Bellman-Ford](learn:graphs/bellman-ford): caminos más cortos cuando los pesos pueden ser negativos.
- [Búsqueda A*](learn:grid-pathfinding/a-star): Dijkstra guiado hacia una meta por una estimación.
