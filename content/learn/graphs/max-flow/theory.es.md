## La idea

Una **red de flujo** es un grafo dirigido en el que cada arista tiene una **capacidad**: cuánto puede pasar por ella, como el agua por una tubería. El **flujo máximo** es lo máximo que puede viajar desde una **fuente** hasta un **sumidero** sin que ninguna arista lleve más que su capacidad y con cada uno de los demás nodos dejando pasar exactamente lo que recibe. En este tema la fuente es el nodo 0 y el sumidero es el nodo `n - 1`.

El ejemplo mantiene una **matriz de capacidades**: `cap[u][v]` es cuánto más puede ir todavía de `u` a `v`, la **capacidad residual**. Cada ronda hace un BFS desde la fuente por las aristas a las que les queda capacidad residual, recorriendo los vecinos de un nodo en **orden de número** (una fila de la matriz). Si llega al sumidero, los padres del BFS dan un camino, un **camino de aumento**. Su **cuello de botella**, la menor capacidad residual a lo largo de él, es cuánto flujo puede añadir. Cada arista del camino pierde esa cantidad y el sentido contrario la **gana**: `cap[v][u] += push`. Cuando el BFS ya no llega al sumidero, el total es el máximo. Tomar siempre un camino más corto con BFS es lo que lo convierte en **Edmonds-Karp**.

La capacidad inversa es la parte ingeniosa. Un camino temprano y voraz puede repartir mal el flujo y usar una arista que un camino posterior necesita. La capacidad inversa permite que un camino posterior **retire ese flujo** y lo mande por otro lado, así que ninguna elección temprana es definitiva. Como la capacidad inversa de `u → v` vive en la celda `cap[v][u]`, la entrada de este tema no admite `u → v` y `v → u` a la vez en el mismo grafo.

Cuando la búsqueda se detiene, los nodos a los que el BFS aún llega y el resto forman un **corte** cuyas aristas de cruce están todas llenas. El **teorema de flujo máximo y corte mínimo** dice que el mayor flujo es igual a la menor capacidad total que habría que quitar para separar la fuente del sumidero.

### La entrada por defecto

Seis nodos: `0 1 5, 0 2 4, 1 3 3, 2 4 6, 1 4 2, 3 5 4, 4 5 7`, con fuente 0 y sumidero 5. La ronda 1 encuentra `0 → 1 → 3 → 5` con cuello de botella 3 (la arista `1 → 3`). La ronda 2 encuentra `0 → 1 → 4 → 5` con cuello de botella 2, que llena `0 → 1`. La ronda 3 encuentra `0 → 2 → 4 → 5` con cuello de botella 4. Ahora las dos aristas que salen de la fuente están llenas, el BFS no puede salir del nodo 0 y la salida es `[9]`: 3 + 2 + 4. El corte son las dos aristas de la fuente, 5 + 4 = 9.

## Pasos

1. Rellena `cap[u][v]` con la capacidad de cada arista; el total empieza en 0.
2. Haz un BFS desde la fuente por las celdas con `cap[u][v] > 0`, apuntando los padres.
3. Si no se llegó al sumidero, devuelve el total.
4. Recorre hacia atrás desde el sumidero para hallar el cuello de botella; réstalo hacia delante, súmalo hacia atrás y súmalo al total. Repite desde el paso 2.

## Coste

| Operación | Coste |
|---|---|
| Ejecución completa | `O(n · m²)`: como mucho `O(n · m)` rondas de aumento, cada una un BFS de `O(m)` (el BFS sobre la matriz del ejemplo cuesta `O(n²)` por ronda) |
| Espacio extra | `O(n²)` para la matriz |

## Errores comunes

- **Olvidar la capacidad inversa.** Sin ella el algoritmo puede quedarse atascado por debajo del máximo tras un primer camino desafortunado.
- **Elegir los caminos con DFS.** Entonces el número de rondas puede crecer con el propio valor del flujo; BFS lo limita por el tamaño del grafo.
- **Sobrescribir aristas paralelas.** Una matriz guarda un número por par; dos aristas `u → v` deben sumar sus capacidades.

## Cuándo usarlo

Usa el flujo máximo para hacer pasar todo lo posible por enlaces limitados (tráfico, tuberías, datos) y para problemas que se reducen a él, como emparejar trabajadores con tareas.

## Temas relacionados

- [Búsqueda en anchura (BFS)](learn:graphs/bfs): la búsqueda que encuentra cada camino de aumento.
- [Representaciones de grafos](learn:graphs/graph-representations): la matriz donde viven las capacidades.
- [Comprobar si es bipartito](learn:graphs/bipartite-check): el emparejamiento bipartito es flujo máximo sobre un grafo de dos lados.
