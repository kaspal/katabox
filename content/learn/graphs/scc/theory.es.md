## La idea

En un grafo dirigido, dos nodos están **fuertemente conectados** cuando cada uno puede llegar al otro siguiendo las flechas. Eso reparte los nodos en **componentes fuertemente conexas** (CFC): grupos en los que todos llegan a todos. Si encoges cada grupo a un solo punto, lo que queda siempre es acíclico, un mapa del tráfico en un solo sentido entre grupos.

Este tema usa el **algoritmo de Kosaraju**: dos pasadas en profundidad. (El algoritmo de Tarjan es la alternativa conocida que encuentra los mismos grupos en una sola pasada con valores de enlace bajo).

La **pasada 1** hace un DFS sobre todos los nodos, empezando una búsqueda nueva desde cada nodo aún no visto, en orden de número, y apunta cada nodo cuando **termina**, después de todos sus descendientes. La **pasada 2** construye el **grafo invertido**, en el que cada flecha apunta al revés, y toma los nodos en **orden de finalización invertido**. Cada nodo sin etiqueta empieza un DFS en el grafo invertido; todo lo que alcanza recibe la etiqueta actual, y el contador de etiquetas sube. Las etiquetas se numeran en el orden en que la pasada 2 encuentra las componentes.

¿Por qué funciona ese orden? Si alguna flecha va del grupo A al grupo B, algún nodo de A termina después de todos los nodos de B. Así que el último nodo en terminar está en un grupo al que no apunta ningún otro grupo. En el grafo invertido, las flechas de ese grupo hacia los demás ahora apuntan hacia dentro, así que la búsqueda que empieza ahí no puede salir de su propio grupo. Una vez etiquetado, el mismo razonamiento vale para lo que queda.

### La entrada por defecto

Ocho nodos, dirigido: tres bucles unidos en un solo sentido, `0 → 1 → 2 → 0`, `3 → 4 → 5 → 3`, `6 → 7 → 6`, más las aristas `2 → 3` y `5 → 6`. La pasada 1 empieza en 0 y baja por `0, 1, 2, 3, 4, 5, 6, 7`; los nodos terminan en el orden 7, 6, 5, 4, 3, 2, 1, 0. La pasada 2 toma primero 0: en el grafo invertido, 0 llega a 2 y después a 1, que forman la componente 0. El siguiente sin etiqueta es 3: llega a 5 y a 4, pero su arista invertida hacia 2 lleva a un nodo ya etiquetado y se detiene ahí, así que `{3, 4, 5}` es la componente 1. Por último, 6 llega a 7: componente 2.

La salida es `[0, 0, 0, 1, 1, 1, 2, 2]`, la etiqueta de componente de cada nodo.

## Pasos

1. Pasada 1: para cada nodo `u` en orden, si no se ha visto, haz un DFS desde él; añade cada nodo a `finished` cuando termina su llamada.
2. Construye el grafo invertido: por cada arista `u → v`, añade `v → u`.
3. Pasada 2: para cada nodo en el orden de `finished` invertido, si no tiene etiqueta, haz un DFS en el grafo invertido dando la etiqueta actual a cada nodo sin etiqueta que alcances; después suma 1 a la etiqueta.
4. Devuelve las etiquetas.

## Coste

| Operación | Coste |
|---|---|
| Dos pasadas de DFS y la inversión | `O(n + m)` |
| Espacio extra | `O(n + m)`: el grafo invertido, las marcas, las etiquetas y la lista de finalización |

## Errores comunes

- **Usar el grafo original en la pasada 2.** Seguir las flechas hacia delante desde el primer nodo arrastraría a su componente los grupos que están aguas abajo.
- **Usar el orden de descubrimiento en vez del de finalización.** Solo el orden de finalización garantiza que los grupos de aguas arriba salen primero.
- **Recursión profunda.** Una cadena larga hace que las dos pasadas se llamen una vez por nodo; en grafos grandes, usa una pila explícita.

## Cuándo usarlo

Usa las CFC para encontrar grupos de estados que se alcanzan entre sí, dependencias circulares entre módulos, o para simplificar un grafo dirigido a su mapa acíclico de grupos antes de aplicar un orden topológico. También son el núcleo del método para el rompecabezas lógico 2-SAT.

## Temas relacionados

- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): las dos pasadas son un DFS normal.
- [Orden topológico](learn:graphs/topological-sort): el mapa de grupos siempre está listo para él.
- [Componentes conexas](learn:graphs/connected-components): la versión no dirigida, con una sola pasada.
- [Detección de ciclos](learn:graphs/cycle-detection): un grupo con más de un nodo siempre contiene un ciclo.
