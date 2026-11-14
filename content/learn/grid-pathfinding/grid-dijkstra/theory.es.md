## La idea

En algunos mapas un paso no es solo un paso: el barro, la arena o un peaje hacen que entrar en ciertas celdas salga más caro. Aquí entrar en una celda abierta cuesta 1 y en una celda marcada de 2 a 9 cuesta ese número, mientras que el 1 sigue siendo un muro. BFS contaría cada paso como 1, así que puede elegir una ruta corta que atraviesa justo las celdas caras. El **algoritmo de Dijkstra**, en cambio, encuentra la ruta de menor **coste total**.

Guarda el mejor coste conocido de cada celda y una **cola de prioridad** de entradas `(distancia, fila, columna)`: primero la menor distancia y, si empatan, la fila más pequeña y luego la columna más pequeña. La celda que sale de la cola queda fijada: nada encontrado después puede llegar a ella más barato, porque cada ruta pendiente solo suma coste. Si es G, su distancia es la respuesta. Si no, cada vecino (arriba, derecha, abajo, izquierda) que no es un muro se **relaja**: si la distancia de esta celda más el coste de entrar en el vecino mejora lo mejor que tenía el vecino, se apunta y se mete una entrada nueva. Las entradas viejas y mayores de la misma celda se quedan en la cola; cuando sale una está **caducada** y se ignora. Sin camino, la respuesta es `[-1]`.

Python usa `heapq`. TypeScript no trae una cola de prioridad, así que el ejemplo en TS mantiene un arreglo ordenado por `(distancia, fila, columna)` tras cada inserción y saca del frente: de sobra para una cuadrícula pequeña.

### La entrada por defecto

El mapa es `S..99..../.#.99.##./.#.99..#./...99...G`: una ciénaga de dos columnas de ancho parte la cuadrícula de arriba abajo, así que toda ruta tiene que cruzarla y pagar 9 + 9 = 18. Dijkstra fija primero las diez celdas abiertas a la izquierda de la ciénaga, con distancias de 0 a 5. Entrar en la ciénaga desde ellas da a su primera columna 11, 12, 13 y 14 de arriba abajo; cruzar la segunda columna cuesta otros 9 (20 arriba). Desde ahí las rutas más baratas recorren las celdas de coste 1 que quedan hasta G.

Se fijan las 31 celdas abiertas, G la última; en este mapa no sale ninguna entrada caducada. La salida es `[27]`: once celdas en las que se entra, nueve a 1 y dos a 9. BFS en el mismo mapa responde 11, el número de pasos.

## Pasos

1. Pon la distancia del inicio a 0, las demás a "desconocida", y mete `(0, fila de inicio, columna de inicio)`.
2. Saca la entrada más pequeña `(d, r, c)`. Si `d` es mayor que la mejor distancia de la celda, ignórala.
3. Si la celda es G, devuelve `d`.
4. Para cada vecino dentro de la cuadrícula que no sea un muro: si es desconocido o `d` más su coste de entrada es menor que su distancia, apunta la distancia nueva y mete la entrada.
5. Si la cola se vacía antes, devuelve `-1`.

## Coste

| Caso | Coste |
|---|---|
| Ejecución completa con un montículo binario | `O(R·C · log(R·C))`: hasta cuatro inserciones por celda, cada una `O(log(R·C))` |
| Espacio extra | `O(R·C)` para las distancias y la cola |

El arreglo ordenado de TS cuesta más por inserción; un programa real usaría un montículo binario.

## Errores comunes

- **Sumar el coste de la celda de la que sales.** El coste es de la celda en la que entras; el de la celda de inicio nunca se paga.
- **Parar cuando se alcanza G al relajar.** Solo sacarla de la cola fija una celda; todavía puede aparecer una ruta más barata antes de que G salga de la cola.
- **Olvidar la comprobación de caducadas.** Procesar otra vez una entrada vieja no estropea la respuesta, pero desperdicia trabajo.

## Cuándo usarlo

Úsalo para mapas de juegos con terreno, planificación de rutas con peajes y cualquier cuadrícula en la que las celdas cuesten cantidades distintas y no negativas. En el modo comparación, hazlo competir con BFS en un mapa con pesos para ver respuestas distintas, o con A* para ver cuántas celdas menos fija A* para el mismo coste.

## Temas relacionados

- [Caminos más cortos de Dijkstra](learn:graphs/dijkstra): el mismo algoritmo en un grafo cualquiera.
- [Montículo binario y cola de prioridad](learn:trees/binary-heap): la cola que hay detrás del algoritmo.
- [Búsqueda en anchura en una cuadrícula](learn:grid-pathfinding/grid-bfs): el caso en el que cada paso cuesta 1.
- [Búsqueda A*](learn:grid-pathfinding/a-star): Dijkstra guiado hacia G por una estimación.
