## La idea

Dijkstra se extiende por igual en todas las direcciones, incluso alejándose de la meta. **A\*** le da sentido de la orientación. Para cada celda suma dos números: `g`, el coste más barato encontrado hasta ahora desde S, y `h`, una estimación de lo que falta. Aquí `h` es la **distancia Manhattan** a G: filas de diferencia más columnas de diferencia. La cola se ordena por `f = g + h`, luego por `h` (si empatan, se prefiere la celda más cercana a G) y luego por fila y columna.

La estimación nunca **sobrestima**: cada paso cuesta al menos 1 y acerca como mucho una fila o una columna, así que llegar a G desde una celda cuesta al menos `h`. Además es **consistente**: un paso cambia `h` como mucho en 1 y cuesta al menos 1, así que `f` nunca baja a lo largo de una ruta. Juntas, estas dos propiedades garantizan que cuando G sale de la cola su `g` es el coste más barato, exactamente lo que devuelve Dijkstra. El resto es Dijkstra: relajar los vecinos (arriba, derecha, abajo, izquierda) a través de `g`, meter `(g + h, h, fila, columna)` e ignorar las entradas caducadas cuyo `g` (`f - h`) supera el mejor de la celda. Sin camino, la respuesta es `[-1]`.

### La entrada por defecto

El mapa es el laberinto de BFS, `S.#....../.##.###.#/....#...#/##.##.#.#/...#..#.G`, y cada celda abierta cuesta 1. S está a 4 filas y 8 columnas de G, así que `h = 12` y `f = 12`. Un paso hacia G suma 1 a `g` y quita 1 a `h`, así que `f` sigue en 12; un paso que se aleja lo sube en 2. A\* saca nueve celdas con `f = 12`: S, el callejón sin salida a su derecha, el borde izquierdo hasta la fila 2, la fila 2 hasta el muro y el hueco de la columna 2 hasta la última fila. Después los muros obligan a elegir con `f = 14`: la celda de la última fila a la izquierda de ese hueco (menor `h`) y la celda que hay encima del final del pasillo de la fila 2. La ruta tiene que subir dos filas, alejándose de G, así que todas las celdas que quedan tienen `f = 16`.

A\* saca 23 celdas de la cola; Dijkstra en el mismo mapa saca 26. La salida es `[16]`, la misma que dan BFS y Dijkstra. En el ejemplo predefinido de la ciénaga la diferencia es mayor: salen 20 entradas (una caducada) frente a las 31 de Dijkstra.

## Pasos

1. Pon el `g` del inicio a 0 y mete `(h, h, fila de inicio, columna de inicio)`.
2. Saca la entrada más pequeña `(f, h, r, c)`. Si `f - h` es mayor que el `g` de la celda, ignórala.
3. Si la celda es G, devuelve su `g`.
4. Para cada vecino dentro de la cuadrícula que no sea un muro: si su `g` es desconocido o `g + coste de entrada` lo mejora, apúntalo y mete `(nuevo g + h, h, fila, columna)`.
5. Si la cola se vacía antes, devuelve `-1`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(longitud del camino)` con una `h` perfecta: solo salen de la cola las celdas de la ruta |
| Caso típico | menos celdas que Dijkstra |
| Peor caso | `O(R·C · log(R·C))`: `h` no ayuda, por ejemplo cuando G está tapiada |
| Espacio extra | `O(R·C)` para `g` y la cola |

## Errores comunes

- **Una estimación que sobrestima.** Duplicar la distancia Manhattan, por ejemplo, puede hacer que A\* devuelva una ruta que no es la más barata.
- **Parar cuando se mete G en la cola.** Igual que en Dijkstra, solo sacar G de la cola hace definitivo su coste.
- **Movimientos en diagonal con distancia Manhattan.** Si se permitieran diagonales, Manhattan podría sobrestimar; los ejemplos solo se mueven en cuatro direcciones.

## Cuándo usarlo

Usa A\* para un inicio y una meta cuando puedas estimar barato el coste restante: personajes de juegos, robots y planificadores de rutas. Hazlo competir con Dijkstra en el modo comparación: los dos dan el mismo coste, pero A\* suele fijar menos celdas.

## Temas relacionados

- [Dijkstra en una cuadrícula con pesos](learn:grid-pathfinding/grid-dijkstra): A\* con `h = 0`.
- [Búsqueda voraz primero el mejor](learn:grid-pathfinding/greedy-best-first): solo `h`, más rápida pero no siempre la más barata.
- [Búsqueda en anchura en una cuadrícula](learn:grid-pathfinding/grid-bfs): el menor número de pasos cuando cada paso cuesta 1.
- [Montículo binario y cola de prioridad](learn:trees/binary-heap): la cola que hay detrás de la búsqueda.
