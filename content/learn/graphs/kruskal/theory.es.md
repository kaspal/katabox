## La idea

El **algoritmo de Kruskal** construye un árbol de expansión mínima sin hacerlo crecer desde un nodo de partida. Ordena todas las aristas de la más barata a la más cara y recorre la lista: una arista se **toma** si une dos trozos que todavía no están conectados, y se **salta** si sus dos extremos ya están en el mismo trozo, porque entonces cerraría un ciclo.

¿Por qué la arista útil más barata siempre es segura? Divide los nodos en dos grupos cualesquiera. Todo árbol de expansión tiene que usar al menos una arista que cruce entre ellos, y cambiar un cruce más caro por el más barato nunca empeora un árbol. Cuando Kruskal llega a una arista que une dos trozos separados, no hay nada más barato que cruce entre esos trozos (ya los habría unido), así que la arista forma parte de algún árbol óptimo. Esta es la **propiedad del corte**, dicha de manera informal.

Para preguntar rápido "¿mismo trozo?", el ejemplo mantiene una estructura de **unión-búsqueda**. `find(x)` sigue los enlaces al padre hasta la raíz del trozo, y la **división del camino a la mitad** acorta el recorrido de paso: cada nodo visitado pasa a apuntar a su abuelo. Tomar una arista `u – v` hace que la raíz de `u` apunte a la raíz de `v`. La clave de ordenación es `(peso, u, v)`, así que los empates los decide el primer extremo menor y después el segundo menor. La salida es el peso total y después las aristas elegidas como pares `u, v`, en el orden en que se tomaron y tal como estaban escritas en la entrada.

### La entrada por defecto

El mismo anillo con dos cuerdas que en Prim: `0 1 4, 1 2 2, 2 3 5, 3 4 1, 4 5 3, 5 0 2, 0 3 8, 1 4 6`. Ordenada, la lista empieza por `3 4 1`, `1 2 2`, `5 0 2`, `4 5 3`, `0 1 4`. Esas cinco unen cada una dos trozos separados y se toman: los totales van 1, 3, 5, 8, 12, y tras `0 1 4` todos los nodos están en un mismo trozo. El resto, `2 3 5`, `1 4 6` y `0 3 8`, encuentran los dos extremos con la misma raíz y se saltan.

La salida es `[12, 3, 4, 1, 2, 5, 0, 4, 5, 0, 1]`: total 12, el mismo árbol que encuentra Prim, pero sus aristas salen en orden de peso.

## Pasos

1. Da a cada nodo su propio trozo: `parent[x] = x`.
2. Ordena las aristas por `(peso, u, v)`.
3. Para cada arista `u – v`: busca las dos raíces. Si son iguales, sáltala.
4. Si no, haz que una raíz apunte a la otra, suma el peso y apunta la arista.

## Coste

| Operación | Coste |
|---|---|
| Ordenar las aristas | `O(m log m)`, la parte dominante |
| Todas las búsquedas y uniones | casi `O(m)` con la división del camino a la mitad |
| Espacio extra | `O(n + m)`: el arreglo de padres y la lista de aristas ordenada |

## Errores comunes

- **Buscar ciclos recorriendo el grafo cada vez.** Un DFS nuevo por cada arista vuelve cuadrática la ejecución; la unión-búsqueda responde en un tiempo casi constante.
- **Esperar un solo árbol de un grafo no conexo.** Kruskal nunca necesita un nodo de partida, así que devuelve un **bosque de expansión**: un árbol por componente, con menos de `n - 1` aristas en total.
- **Unir nodos en vez de raíces.** Escribir `parent[u] = v` en lugar de enlazar las dos raíces separa trozos que estaban unidos.

## Cuándo usarlo

Usa Kruskal cuando las aristas llegan como una lista, el grafo es disperso o puede no ser conexo. Parar antes de tiempo, antes de las últimas uniones, da grupos: los trozos que quedan son conjuntos cuyos enlaces son todos baratos.

## Temas relacionados

- [Unión-búsqueda (conjuntos disjuntos)](learn:trees/union-find): la estructura que responde "¿mismo trozo?".
- [Árbol de expansión mínima de Prim](learn:graphs/prim): el mismo árbol, crecido desde un nodo de partida con una cola de prioridad.
- [Componentes conexas](learn:graphs/connected-components): los trozos que abarca el bosque de Kruskal.
