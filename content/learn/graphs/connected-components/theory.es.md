## La idea

Un grafo no dirigido puede romperse en trozos separados: grupos de nodos unidos entre sí por caminos, pero sin ninguna arista que llegue a otro grupo. Cada trozo es una **componente conexa**, y etiquetarlas responde a "¿puede `a` llegar a `b`?" con una sola comparación.

El ejemplo usa un **relleno por inundación**. Recorre los nodos en orden de índice. Cuando encuentras uno que aún no tiene etiqueta, empieza una componente nueva: dale la siguiente etiqueta y extiéndela a todo lo que alcance. La extensión usa una **pila** explícita: saca un nodo, y cada vecino sin etiqueta recibe la misma etiqueta y se apila. Cuando la pila se vacía, la componente está completa y el recorrido sigue. Como el recorrido va en orden de índice, las etiquetas se numeran en el orden del nodo más pequeño de cada componente: la componente que contiene el nodo 0 recibe la etiqueta 0, y así sucesivamente.

Un nodo se etiqueta al apilarlo, no al sacarlo, así que nunca se apila dos veces. Cualquier recorrido sirve para extender la etiqueta: con una cola sería BFS, con recursión sería DFS; las etiquetas salen iguales.

### La entrada por defecto

Diez nodos, no dirigido: un camino 0 – 1 – 2, un triángulo 3 – 4 – 5 y una cadena 6 – 7 – 8 – 9 cuya arista central 7 – 8 se escribe la última. El nodo 0 empieza la etiqueta 0 y el relleno llega a 1 y luego a 2. Los nodos 1 y 2 ya tienen etiqueta, así que el recorrido se los salta. El nodo 3 empieza la etiqueta 1: al sacar 3 se etiquetan a la vez 4 y 5. El nodo 6 empieza la etiqueta 2, y el relleno avanza por 7, 8 y 9 a lo largo de la cadena.

La salida es `[0, 0, 0, 1, 1, 1, 2, 2, 2, 2]`: tres componentes.

## Pasos

1. Pon todas las etiquetas a `-1` y el contador de componentes a `0`.
2. Recorre los nodos `s` en orden de índice; sáltate los que ya tienen etiqueta.
3. Si no, da a `s` la etiqueta `count` y apílalo en una pila vacía.
4. Mientras la pila no esté vacía, saca `u`; a cada vecino `v` sin etiqueta dale la etiqueta `count` y apílalo.
5. Cuando la pila se vacía, suma uno a `count` y sigue el recorrido.

## Coste

| Operación | Coste |
|---|---|
| Etiquetado completo | `O(n + m)`: cada nodo se apila una vez y cada lista se lee una vez |
| Espacio extra | `O(n)` para las etiquetas y la pila |

## Errores comunes

- **Olvidar los nodos aislados.** Un nodo sin aristas es una componente por sí solo; el recorrido por todos los nodos lo encuentra.
- **Etiquetar al sacar.** Así un nodo con varios vecinos etiquetados se apila varias veces; no cambia la respuesta, pero desperdicia memoria.
- **Usarlo en grafos dirigidos.** Seguir las flechas en un sentido encuentra lo que alcanza un nodo, no una componente; la alcanzabilidad mutua necesita las [componentes fuertemente conexas](learn:graphs/scc).

## Cuándo usarlo

Úsalo para contar islas, redes o grupos, para comprobar si un grafo es de una sola pieza o para responder muchas preguntas de "¿mismo grupo?" tras una sola pasada. Si las aristas siguen llegando con el tiempo, unión-búsqueda hace el mismo trabajo de forma incremental. Las cuadrículas también son grafos: cada celda es un nodo unido a sus vecinas, y rellenar regiones de una imagen o de un mapa es el mismo algoritmo.

## Temas relacionados

- [Unión-búsqueda (conjuntos disjuntos)](learn:trees/union-find): componentes que crecen arista a arista.
- [Pila](learn:linear/stack): la pila que mueve el relleno.
- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): la forma recursiva de extender una etiqueta.
- [Búsqueda en anchura en una cuadrícula](learn:grid-pathfinding/grid-bfs): la misma idea sobre las celdas de una cuadrícula.
