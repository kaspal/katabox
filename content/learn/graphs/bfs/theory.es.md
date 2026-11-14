## La idea

La **búsqueda en anchura** (BFS) explora un grafo por anillos: primero el nodo de partida, luego cada nodo a una arista, luego cada nodo a dos aristas, y así sucesivamente. Como nunca llega a un nodo por un camino más largo antes que por uno más corto, la primera vez que encuentra un nodo lo hace por un camino con el menor número de aristas. Por eso BFS es el algoritmo de caminos más cortos para grafos **sin pesos**.

El motor es una **cola**: el primero en entrar es el primero en salir. El ejemplo empieza en el nodo 0 con distancia 0. Saca el nodo del frente, mira sus vecinos en el orden de su lista de adyacencia, y cada vecino que aún no se ha visto recibe la distancia "la mía + 1" y entra por el final de la cola. La distancia de un nodo se fija en cuanto se descubre, y eso también lo marca como visto, así que ningún nodo entra dos veces en la cola. Los nodos a los que BFS no llega se quedan con distancia `-1`.

### La entrada por defecto

Nueve nodos forman una cuadrícula de 3 × 3, numerados fila a fila: 0, 1, 2 arriba, 3, 4, 5 en medio y 6, 7, 8 abajo, con aristas entre los nodos que están uno al lado del otro o uno encima del otro. Visitar 0 descubre 1 y 3 (distancia 1). Visitar 1 descubre 2 y 4 (distancia 2); visitar 3 encuentra 0 y 4 ya vistos y descubre 6 (distancia 2). Después 2 descubre 5 y 4 descubre 7 (distancia 3), y al final 5 descubre 8 (distancia 4).

Los nodos salen de la cola como 0, 1, 3, 2, 4, 6, 5, 7, 8, y la salida es `[0, 1, 2, 1, 2, 3, 2, 3, 4]`: la distancia de cada nodo a la esquina superior izquierda, contada en aristas.

## Pasos

1. Pon todas las distancias a `-1`, la del nodo de partida a `0` y mételo en la cola.
2. Mientras la cola no esté vacía, saca el nodo `u` del frente.
3. Para cada vecino `v` de `u`, en el orden de la lista: si la distancia de `v` sigue siendo `-1`, ponla a la distancia de `u` más 1 y añade `v` al final de la cola.
4. Cuando la cola se vacía, cada nodo alcanzable tiene su distancia; el resto sigue en `-1`.

## Coste

| Operación | Coste |
|---|---|
| Búsqueda completa | `O(n + m)`: cada nodo entra en la cola una vez y cada lista se lee una vez |
| Espacio extra | `O(n)` para las distancias y la cola |

## Errores comunes

- **Marcar un nodo al sacarlo de la cola en vez de al meterlo.** Así un nodo puede entrar varias veces, desde vecinos distintos, antes de procesarse, y se desperdicia trabajo y, en grafos densos, memoria.
- **Una cola lenta.** `list.pop(0)` en Python y `shift()` en JavaScript desplazan todos los elementos; usa `collections.deque` o un índice de cabeza, como hacen los ejemplos.
- **Usar BFS con aristas con peso.** Menos aristas no es lo mismo que menor peso total; con pesos necesitas Dijkstra.

## Cuándo usarlo

Usa BFS para el menor número de movimientos, saltos o pasos entre dos estados: rutas más cortas en redes sin pesos, escaleras de palabras, rompecabezas donde cada movimiento cuesta lo mismo y procesos que se extienden, como un contagio o un rumor, donde el número de anillo es el paso de tiempo.

## Temas relacionados

- [Cola y búfer circular](learn:linear/queue-ring-buffer): la cola que mueve la búsqueda.
- [Recorrido por niveles](learn:trees/level-order): el mismo recorrido en un árbol, donde a cada nodo se llega por un solo camino.
- [Caminos más cortos de Dijkstra](learn:graphs/dijkstra): caminos más cortos cuando las aristas tienen peso.
- BFS 0-1 (próximamente): BFS con una bicola cuando los pesos solo son 0 o 1.
