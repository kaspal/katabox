## La idea

En un grafo no dirigido, un **puente** es una arista que, si se quita, parte en dos un trozo conexo, y un **punto de articulación** (o vértice de corte) es un nodo que, si se quita, hace lo mismo. Son los puntos únicos de fallo de una red. Un solo DFS los encuentra todos.

El DFS numera los nodos según entra en ellos: `tin[u]` es el **tiempo de entrada**. También calcula el **enlace bajo** `low[u]`: el menor tiempo de entrada al que puede llegar el subárbol DFS de `u` bajando por sus aristas del árbol más **una arista de retroceso** hacia un nodo ya visitado. El ejemplo pone `low[u] = tin[u]` al entrar, lo baja a `tin[v]` por cada arista de retroceso `u – v` (saltándose la arista hacia el padre) y, cuando vuelve cada hijo `v`, lo baja a `low[v]`.

Una vez conocido `low`, las pruebas son sencillas. Para una arista del árbol `u – v`:

- Si `low[v] > tin[u]`, nada por debajo de `v` puede volver a subir hasta `u` o más arriba, así que `u – v` es un **puente**.
- Si `low[v] >= tin[u]` y `u` no es la raíz, el subárbol de `v` cuelga solo de `u`, así que `u` es un **vértice de corte**.
- La **raíz** de un DFS solo es vértice de corte si tiene **dos o más hijos en el DFS**: sin ella no podrían llegar el uno al otro.

La salida da los puentes como pares ordenados `(menor, mayor)`, después un separador `-1` y después los vértices de corte en orden ascendente.

### La entrada por defecto

Siete nodos, no dirigido: un triángulo `0 – 1 – 2`, la arista `2 – 3`, un triángulo `3 – 4 – 5` y una cola `5 – 6`. El DFS entra en los nodos en orden de número, así que `tin` va de 0 a 6. En el nodo 2, la arista de retroceso hacia 0 da `low[2] = 0`; en el nodo 5, la arista de retroceso hacia 3 da `low[5] = 3`, y eso se propaga a `low[4] = 3` y `low[3] = 3`. El nodo 6 es una hoja con `low[6] = 6`.

De vuelta en 5: `low[6] = 6 > tin[5] = 5`, así que `5 – 6` es un puente y 5 un vértice de corte. En 3: `low[4] = 3 >= tin[3]`, así que 3 es un vértice de corte, aunque `3 – 4` no es un puente. En 2: `low[3] = 3 > tin[2] = 2`, así que `2 – 3` es un puente y 2 un vértice de corte. La raíz 0 tiene un solo hijo en el DFS, así que no es vértice de corte. La salida es `[2, 3, 5, 6, -1, 2, 3, 5]`.

## Pasos

1. Para cada nodo aún no visitado, empieza un DFS sin padre.
2. Al entrar en `u`: `tin[u] = low[u] = timer` y suma 1 al contador.
3. Para cada vecino `v` que no sea el padre: si ya está visitado, `low[u] = min(low[u], tin[v])`; si no, visítalo, cuenta un hijo, pon `low[u] = min(low[u], low[v])` y aplica las pruebas de puente y de corte.
4. Tras el bucle, una raíz con dos o más hijos es un vértice de corte.

## Coste

| Operación | Coste |
|---|---|
| Un DFS | `O(n + m)` |
| Espacio extra | `O(n)`: `tin`, `low` y la recursión |

## Errores comunes

- **Tratar la arista hacia el padre como una arista de retroceso.** Entonces todos los `low` bajan hasta el padre y nunca se encuentra un puente.
- **Aristas paralelas.** El ejemplo se salta al padre por nodo, así que una arista doble `u – v` se saltaría dos veces y se daría por puente sin serlo; para admitirlas, sáltala por identificador de arista.
- **Aplicar la prueba `>=` de corte a la raíz.** La raíz no tiene ningún tiempo de entrada por debajo del suyo, así que necesita contar sus hijos.

## Cuándo usarlo

Úsalo para encontrar puntos débiles: carreteras o cables cuyo fallo corta una red, o enrutadores que no pueden caerse.

## Temas relacionados

- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): el recorrido que fija los tiempos de entrada.
- [Componentes fuertemente conexas](learn:graphs/scc): el pariente dirigido, también construido sobre DFS.
- [Componentes conexas](learn:graphs/connected-components): lo que parte un puente o un vértice de corte.
