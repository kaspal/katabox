## La idea

Un **grafo** es un conjunto de nodos unidos por aristas. Aquí los nodos se numeran de `0` a `n − 1`, y cada arista `u v w` une `u` con `v` con un peso `w` (un grafo sin pesos usa simplemente `w = 1`). En un grafo **dirigido** la arista va en un solo sentido; en uno **no dirigido** sirve en los dos, así que se escribe dos veces: una de `u` a `v` y otra de `v` a `u`.

Antes de que un algoritmo pueda recorrer un grafo, necesita las aristas guardadas en una forma que pueda consultar. Dos formas cubren casi todos los casos:

- **Lista de adyacencia**: una lista por nodo, con los vecinos a los que tiene aristas, en el orden en que llegan las aristas.
- **Matriz de adyacencia**: una cuadrícula de `n × n` donde la celda `[u][v]` guarda el peso de la arista de `u` a `v`, y `0` significa que no hay arista.

El ejemplo construye las dos en una sola pasada por la lista de aristas y después escribe la lista de cada nodo (su longitud y luego sus vecinos) seguida de la matriz, fila a fila.

### La entrada por defecto

Cinco nodos, no dirigido, con las aristas `0 1 4, 0 2 1, 1 3 2, 2 3 5, 3 4 3`. Las listas quedan así: 0: [1, 2], 1: [0, 3], 2: [0, 3], 3: [1, 2, 4] y 4: [3]. El nodo 3 aparece en tres aristas, así que su lista es la más larga; el nodo 4 solo tiene una. Las cinco aristas ocupan 10 entradas de las listas, dos por arista.

La matriz tiene 25 celdas. Diez guardan un peso, por ejemplo `[0][1] = [1][0] = 4` y `[3][4] = [4][3] = 3`, y las otras 15 valen `0`. La matriz es simétrica porque el grafo es no dirigido.

## Pasos

1. Crea `n` listas vacías y una matriz de `n × n` llena de ceros.
2. Para cada arista `u v w`, añade `v` a la lista de `u` y pon `matrix[u][v] = w`.
3. Si el grafo es no dirigido, añade también `u` a la lista de `v` y pon `matrix[v][u] = w`.
4. Cuando se acaban las aristas, las dos estructuras describen el mismo grafo.

## Coste

| Operación | Coste |
|---|---|
| Construir la lista | `O(n + m)` para `n` nodos y `m` aristas |
| ¿Hay una arista `u → v`? | `O(1)` en la matriz, `O(grado de u)` en la lista |
| Visitar todos los vecinos de `u` | `O(grado de u)` en la lista, `O(n)` en la matriz |
| Espacio | `O(n + m)` para la lista, `O(n²)` para la matriz |

## Errores comunes

- **Olvidar la arista de vuelta.** En un grafo no dirigido, escribir solo `u → v` deja a `v` sin poder llegar a `u`, y los recorridos se saltan nodos sin avisar.
- **Usar `0` como "no hay arista" cuando los pesos pueden ser `0`.** La matriz no distingue una arista gratis de una que no existe; en ese caso usa otra marca, como `None` o infinito.
- **Una matriz para un grafo grande y disperso.** Cien mil nodos son diez mil millones de celdas, aunque haya pocas aristas.

## Cuándo usarlo

Elige la lista de adyacencia por defecto: la mayoría de los algoritmos de grafos visitan todos los vecinos de un nodo, y la lista lo hace en un tiempo proporcional a las aristas que existen. Elige la matriz cuando el grafo es pequeño o denso, o cuando preguntas sin parar "¿está `u` unido a `v`?", como en Floyd-Warshall (próximamente).

## Temas relacionados

- [Tabla hash: encadenamiento](learn:hashing/separate-chaining): otro arreglo de listas.
- [Búsqueda en anchura (BFS)](learn:graphs/bfs): el primer recorrido sobre una lista de adyacencia.
- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): el otro recorrido básico.
- Floyd-Warshall (próximamente): caminos más cortos entre todos los pares sobre una matriz.
