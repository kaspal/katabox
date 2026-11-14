## La idea

Una cuadrícula es un grafo disfrazado: cada celda abierta es un nodo, unido a las celdas que tiene justo encima, a la derecha, debajo y a la izquierda. La **búsqueda en anchura** sobre ese grafo se extiende desde el inicio por anillos, paso a paso, así que la primera vez que llega a una celda lo hace con el menor número posible de pasos. Por eso es la herramienta adecuada para "¿cuántos movimientos hacen falta para ir de S a G?" cuando todos los movimientos cuestan lo mismo.

El ejemplo guarda una cuadrícula de distancias, todas a `-1` salvo el `0` del inicio, y una **cola**: el primero en entrar es el primero en salir. Saca la celda del frente; si es G, responde con la distancia de G. Si no, prueba los cuatro vecinos, siempre en el orden arriba, derecha, abajo, izquierda. Un vecino que está dentro de la cuadrícula, no es un muro y sigue en `-1` recibe "mi distancia + 1" y entra por el final de la cola. Poner la distancia lo marca como visto, así que ninguna celda entra dos veces en la cola. Si la cola se vacía antes, G es inalcanzable y la respuesta es `[-1]`.

BFS cuenta **pasos**, no coste: para BFS una celda con peso (de 2 a 9) es solo una celda abierta.

### La entrada por defecto

El mapa es `S.#....../.##.###.#/....#...#/##.##.#.#/...#..#.G`: cinco filas, nueve columnas, S en la esquina superior izquierda y G en la inferior derecha. Justo después de S sale de la cola su vecino de la derecha (se encontró antes que el de abajo), pero los muros lo encierran: un callejón sin salida a distancia 1. La ruta buena baja por el borde izquierdo hasta la fila 2, va a la derecha hasta la distancia 5, sube por la columna de índice 3 hasta la fila de arriba (distancia 7), sigue por arriba hasta la columna de índice 7 (distancia 11), baja por esa columna hasta la fila 4 (distancia 15) y da un paso a la derecha hasta G.

Por el camino BFS también llena los callejones sin salida: la esquina inferior izquierda (distancia 8), la esquina superior derecha (12) y el pasillo corto que sale de la columna 7 hacia la izquierda en la fila 2. Saca 25 celdas de la cola, G la última, y la salida es `[16]`.

## Pasos

1. Pon todas las distancias a `-1`, la del inicio a `0`, y mete el inicio en la cola.
2. Saca la celda del frente. Si es G, devuelve su distancia.
3. Para cada vecino (arriba, derecha, abajo, izquierda): si está dentro de la cuadrícula, no es un muro y sigue en `-1`, pon su distancia a la de esta celda más 1 y añádelo al final de la cola.
4. Repite; si la cola se vacía sin llegar a G, devuelve `-1`.

## Coste

| Caso | Coste |
|---|---|
| Búsqueda completa | `O(R·C)`: cada celda entra en la cola como mucho una vez y revisa cuatro vecinos |
| Espacio extra | `O(R·C)` para las distancias y la cola |

## Errores comunes

- **Marcar las celdas al sacarlas de la cola.** Así una celda puede entrar en la cola desde hasta cuatro vecinos. Márcala en cuanto se descubre, como hacen los ejemplos.
- **Confundir filas y columnas.** Arriba y abajo cambian la fila, izquierda y derecha la columna; comprueba `0 ≤ r < R` y `0 ≤ c < C` por separado.
- **Fiarse de BFS con pesos.** En el mapa por defecto de Dijkstra BFS responde 11 pasos, pero la ruta más barata cuesta 27.

## Cuándo usarlo

Usa BFS en cuadrículas para el menor número de movimientos en laberintos, tableros de juego, suelos por los que se mueve un robot y procesos que se extienden, cuando cada paso cuesta lo mismo. En el modo comparación puedes hacerlo competir con otra búsqueda sobre el mismo mapa: en un mapa sin pesos empata con Dijkstra en la respuesta y gasta menos trabajo por celda.

## Temas relacionados

- [Búsqueda en anchura (BFS)](learn:graphs/bfs): el mismo algoritmo en un grafo cualquiera.
- [Cola y búfer circular](learn:linear/queue-ring-buffer): la cola que mueve la búsqueda.
- [Dijkstra en una cuadrícula con pesos](learn:grid-pathfinding/grid-dijkstra): cuando las celdas cuestan cantidades distintas.
- [Búsqueda A*](learn:grid-pathfinding/a-star): una búsqueda guiada hacia G.
- [BFS 0-1](learn:graphs/zero-one-bfs): BFS con una bicola cuando los pasos cuestan 0 o 1.
