## La idea

Un **árbol de expansión mínima** (AEM) une todos los nodos de un grafo no dirigido con pesos usando un subconjunto de sus aristas, sin ciclos y con el menor peso total posible. Imagina que tiendes cable entre pueblos: todos los pueblos deben quedar conectados y pagas por cada tramo de cable.

El **algoritmo de Prim** hace crecer el árbol desde un nodo, aquí el nodo 0. En cada paso añade la arista más barata que une un nodo que ya está en el árbol con uno de fuera. Esa arista nunca puede ser un error: sea como sea el mejor árbol, tiene que cruzar de "dentro" a "fuera" en algún punto, y el cruce más barato es al menos tan bueno como cualquier otro.

La herramienta es una **cola de prioridad** de ternas `(peso, nodo, origen)`, primero la menor y, si empatan, decide el nodo y después de dónde viene. El nodo de partida entra como `(0, 0, 0)`. Cuando sale una terna y su nodo ya está en el árbol, está **caducada** y se ignora; si no, el nodo entra en el árbol, su peso se suma al total y se mete cada arista hacia un nodo que sigue fuera. La salida es el peso total y después los nodos en el orden en que entraron.

Python usa `heapq`. TypeScript no tiene cola de prioridad, así que el ejemplo en TS mantiene un arreglo de ternas ordenado tras cada inserción, lo que basta para una docena de nodos.

### La entrada por defecto

Seis nodos, no dirigido: `0 1 4, 1 2 2, 2 3 5, 3 4 1, 4 5 3, 5 0 2, 0 3 8, 1 4 6`: un anillo de seis con dos cuerdas. Desde 0 la cola recibe `(4, 1, 0)`, `(2, 5, 0)` y `(8, 3, 0)`. La más barata es el nodo 5 (2). El nodo 5 ofrece el nodo 4 con peso 3, así que 4 entra a continuación (total 5), y después 3 por la arista `3 – 4` de peso 1 (total 6). Ahora el cruce más barato es `0 – 1` con 4 (total 10), y al final `1 – 2` con 2 (total 12). Las entradas `(5, 2, 3)`, `(6, 1, 4)` y `(8, 3, 0)` están caducadas cuando salen.

La salida es `[12, 0, 5, 4, 3, 1, 2]`. Solo quedan fuera la arista del anillo `2 – 3` (5) y las dos cuerdas.

## Pasos

1. Mete `(0, inicio, inicio)`.
2. Saca la terna más pequeña `(w, v, origen)`. Si `v` ya está en el árbol, ignórala.
3. Añade `v` al árbol, ponlo al final del orden y suma `w` al total.
4. Para cada arista `v – x` con peso `wx` en la que `x` está fuera del árbol, mete `(wx, x, v)`.
5. Repite hasta que la cola esté vacía.

## Coste

| Operación | Coste |
|---|---|
| Ejecución completa con un montículo binario | `O(m log m)`: cada arista se mete como mucho una vez y se saca como mucho una vez |
| Espacio extra | `O(n + m)`: las marcas de "en el árbol" más la cola |

## Errores comunes

- **Un grafo no conexo.** Prim solo cruza aristas desde el árbol que está creciendo, así que abarca **la componente del nodo 0** e ignora el resto sin avisar. Para un bosque de expansión, vuelve a empezar desde cada nodo no alcanzado o usa Kruskal.
- **Grafos dirigidos.** La idea de árbol de expansión necesita aristas no dirigidas; las versiones dirigidas son otro problema, más difícil.
- **Olvidar ignorar las entradas caducadas.** Sin la comprobación de "en el árbol", un nodo entraría dos veces y su peso se contaría dos veces.

## Cuándo usarlo

Usa Prim para la red más barata que lo conecta todo: cables, tuberías, carreteras, o para agrupar puntos cortando las aristas más caras del árbol. Encaja con grafos densos dados como listas o matrices de adyacencia.

## Temas relacionados

- [Árbol de expansión mínima de Kruskal](learn:graphs/kruskal): el mismo árbol construido arista a arista, de la más barata a la más cara.
- [Caminos más cortos de Dijkstra](learn:graphs/dijkstra): la misma cola, pero ordenada por la distancia total en vez de por una sola arista.
- [Montículo binario y cola de prioridad](learn:trees/binary-heap): la cola que hay detrás del algoritmo.
