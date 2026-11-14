## La idea

Un grafo **dirigido** tiene un ciclo cuando puedes seguir sus flechas desde algún nodo y volver a él. Una búsqueda en profundidad encuentra uno dando a cada nodo uno de tres colores:

- **blanco**: aún no visitado;
- **gris**: ya ha entrado pero no ha terminado; los nodos grises son justo el camino actual en la pila de llamadas;
- **negro**: terminado; todo lo que se alcanza desde él ya se ha explorado.

Mientras se explora `u`, una arista hacia un nodo **gris** apunta a un nodo del camino actual. Es una **arista de retroceso**: el camino desde ese nodo hasta `u`, más la arista, cierra un bucle, así que el grafo tiene un ciclo. Una arista hacia un nodo **negro** es inofensiva: ese nodo se exploró por completo y ninguno de sus caminos volvía al camino actual, así que ir allí otra vez no puede cerrar un bucle. Las aristas hacia nodos blancos se siguen como siempre.

El ejemplo lanza una búsqueda desde cada nodo que sigue blanco, en orden de índice, y escribe `[1]` en cuanto encuentra una arista de retroceso, o `[0]` cuando todos los nodos se vuelven negros.

### La entrada por defecto

Siete nodos, dirigido, con las aristas `0→1, 1→2, 0→3, 3→4, 4→5, 5→3, 2→6`. Desde 0, DFS pasa a gris 1, 2 y 6. El nodo 6 no tiene aristas y se vuelve negro; también 2 y 1 al retroceder. Después 0 sigue su segunda arista hasta 3, y el camino crece a 0, 3, 4, 5, todos grises. La arista de 5 lleva a 3, que es gris: una arista de retroceso, y el bucle 3 → 4 → 5 → 3. La salida es `[1]`.

## Pasos

1. Pinta todos los nodos de blanco.
2. Para cada nodo `s` en orden de índice, si es blanco, llama a `visit(s)`.
3. `visit(u)`: pinta `u` de gris. Para cada arista `u → v`: si `v` es gris, informa de un ciclo; si `v` es blanco, visítalo, e informa de un ciclo si esa visita encontró uno.
4. Cuando se acaban las aristas de `u`, píntalo de negro y devuelve "sin ciclo".
5. Si todas las búsquedas terminan sin encontrar un ciclo, el grafo no tiene ninguno.

## Coste

| Operación | Coste |
|---|---|
| Comprobación completa | `O(n + m)`: cada nodo se vuelve gris y negro una vez y cada arista se mira una vez |
| Espacio extra | `O(n)` para los colores y la pila de llamadas |

## Errores comunes

- **Usar solo dos colores.** Con solo "visto" y "no visto", una arista hacia una rama ya terminada parece un ciclo. En un grafo como `0→1, 0→2, 1→2`, la arista `1→2` llega a un nodo visto, pero no hay bucle.
- **Empezar solo desde el nodo 0.** Un ciclo puede estar en una parte del grafo a la que 0 no llega; lanza una búsqueda desde cada nodo blanco.
- **Los grafos no dirigidos funcionan distinto.** Allí cada arista lleva directamente de vuelta al nodo del que vienes, y eso no es un ciclo. Tienes que recordar el padre de cada nodo e ignorar esa arista, o usar unión-búsqueda e informar de un ciclo cuando una arista une dos nodos que ya están en el mismo grupo.

## Cuándo usarlo

Úsalo antes de fiarte de un grafo de dependencias: los sistemas de compilación, los requisitos previos de un curso, las fórmulas de una hoja de cálculo y los calendarios de tareas se rompen cuando se cuela un ciclo. También es la comprobación que te dice que existe un orden topológico.

## Temas relacionados

- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): el recorrido que pinta los colores.
- [Orden topológico](learn:graphs/topological-sort): ordenar un grafo que no tiene ciclos.
- [Detección de ciclos de Floyd](learn:linear/floyd-cycle): encontrar un bucle en una lista enlazada con dos punteros.
- [Unión-búsqueda (conjuntos disjuntos)](learn:trees/union-find): detectar ciclos en grafos no dirigidos arista a arista.
