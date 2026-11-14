## La idea

A\* tiene en cuenta dos cosas: lo que ha costado una ruta hasta ahora (`g`) y lo lejos que parece quedar todavía (`h`). La **búsqueda voraz primero el mejor** descarta la primera. Siempre saca la celda que parece más cercana a G, solo por la **distancia Manhattan** `h`, como si no importara lo que cuesta llegar hasta allí. La cola guarda entradas `(h, fila, columna)`: primero la menor `h` y, si empatan, la fila y luego la columna más pequeñas.

No hay actualizaciones de distancia. Cuando un vecino (arriba, derecha, abajo, izquierda) se descubre por primera vez, se marca apuntando lo que se ha pagado para llegar a él: lo de la celda actual más el coste de entrar en el vecino (1 para una celda abierta, de 2 a 9 para una con peso). Una celda marcada no se vuelve a tocar, aunque más tarde aparezca un camino más barato hasta ella. Cuando G sale de la cola, la respuesta es lo pagado por la ruta que la descubrió. Si la cola se vacía, la respuesta es `[-1]`.

Eso hace que la búsqueda voraz sea rápida cuando el camino está despejado, pero ciega a dos cosas: los muros que hay entre una celda y G, y los costes. La ruta que da es válida, pero no necesariamente la más barata.

### La entrada por defecto

El mapa "trampa", `......../.#####../.....#../S....#.G/.....#../.#####../........`, tiene una caja de muros abierta hacia S. G está justo enfrente, así que `h` baja de 7 a 3 mientras la búsqueda avanza por la fila 3 hacia el interior de la caja. Allí el muro la frena, y todas las celdas que quedan dentro parecen peores, así que la búsqueda llena toda la bolsa mientras sale de ella. Solo entonces sube por el borde izquierdo, recorre la fila de arriba con `h` bajando otra vez y baja hasta G.

Saca 28 celdas de la cola; BFS necesita 43 en el mismo mapa. La salida es `[13]`: 3 hacia arriba, 7 de lado y 3 hacia abajo. Aquí resulta que ese es también el coste más barato, pero nada lo garantiza. En el mapa diminuto `S9G/...`, la búsqueda voraz entra en el 9 porque está justo al lado de G, y paga 10; la ruta más barata lo rodea por 4. La entrada «Un carril de peaje» muestra lo mismo en un mapa mayor: la búsqueda voraz paga 14 donde bastaría con 8.

## Pasos

1. Marca el inicio con 0 pagado y mete `(h, fila de inicio, columna de inicio)`.
2. Saca la entrada con la menor `h`. Si es G, devuelve lo pagado para llegar a ella.
3. Para cada vecino dentro de la cuadrícula que no sea un muro ni esté marcado: márcalo con lo de esta celda más su coste de entrada y mete `(h, fila, columna)`.
4. Si la cola se vacía antes, devuelve `-1`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(longitud del camino)`: G se ve en línea recta, sin obstáculos |
| Caso típico | a menudo pocas celdas, pero el camino no siempre es el más barato |
| Peor caso | `O(R·C · log(R·C))`: cada celda entra en la cola una vez |
| Espacio extra | `O(R·C)` para las marcas y la cola |

## Errores comunes

- **Esperar la ruta más barata.** La búsqueda voraz ignora lo que ha pagado; usa A\* o Dijkstra cuando importa el coste.
- **Callejones sin salida con forma de trampa.** Una bolsa orientada hacia el inicio atrae a la búsqueda, que tiene que vaciarla antes de probar otra cosa.
- **Leer la respuesta como una distancia.** Es el coste de una ruta encontrada, no un mínimo garantizado.

## Cuándo usarlo

Úsala cuando cualquier ruta sirve y la velocidad importa más que el coste: comprobaciones rápidas en mapas abiertos o como primera aproximación. Hazla competir con A\* en el modo comparación: en mapas abiertos a menudo explora menos celdas, y en mapas con pesos puedes ver cómo paga más.

## Temas relacionados

- [Búsqueda A\*](learn:grid-pathfinding/a-star): vuelve a sumar el coste acumulado y obtiene la ruta más barata.
- [Dijkstra en una cuadrícula con pesos](learn:grid-pathfinding/grid-dijkstra): solo coste, sin estimación.
- [Búsqueda en anchura en una cuadrícula](learn:grid-pathfinding/grid-bfs): solo pasos, sin estimación.
- [Montículo binario y cola de prioridad](learn:trees/binary-heap): la cola que hay detrás de la búsqueda.
