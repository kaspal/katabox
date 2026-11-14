## La idea

Una fila de casillas contiene números. Desde la casilla `i` puedes saltar hacia delante cualquier distancia de 1 hasta el número escrito en ella; un 0 significa que no puedes moverte desde esa casilla. Empezando en la casilla 0, ¿puedes llegar a la última?

Probar cada salto se ramifica enseguida. La visión voraz necesita un solo número: el **alcance máximo**, el índice más alto al que puede saltar cualquier casilla vista hasta ahora. Recorre de izquierda a derecha. Si el índice actual está más allá del alcance, ninguna casilla anterior puede llegar aquí, así que el recorrido se **atasca**. Si no, la casilla `i` llega a `i + salto`; si eso es más lejos, el alcance crece. Toda casilla hasta el alcance es alcanzable, porque el salto que lo fijó puede detenerse en cualquier punto del camino.

La salida es `[1, alcance]` cuando el recorrido atraviesa toda la fila, con el alcance limitado al último índice, o `[0, alcance]` cuando se atasca, el índice más lejano al que pudo llegar.

### La entrada por defecto

Diez casillas: `2, 3, 1, 1, 0, 2, 4, 0, 1, 1`. La casilla 0 llega a 2. La casilla 1 llega a `1 + 3 = 4`. Las casillas 2, 3 y 4 llegan a 3, 4 y 4: no mejoran. La casilla 5 está más allá del alcance de 4: atasco.

La salida es `[0, 4]`. El 0 de la casilla 4 es una trampa: todos los caminos acaban ahí, y el 4 de la casilla 6, que habría llegado a la última casilla, nunca se visita.

## Pasos

1. Pon `alcance = 0`.
2. Para cada índice `i` de izquierda a derecha: si `i > alcance`, devuelve `[0, alcance]`.
3. Si `i + saltos[i] > alcance`, pon `alcance = i + saltos[i]`.
4. Tras el bucle, devuelve `[1, min(alcance, n - 1)]`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n)`: una pasada, trabajo constante por casilla |
| Espacio extra | `O(1)`: solo el alcance |

## Errores comunes

- **Mirar solo el salto desde la casilla 0.** El alcance sale de todas las casillas visitadas hasta ahora, no solo de la última.
- **El orden de las comprobaciones.** Comprueba `i > alcance` antes de usar la casilla `i`; una casilla fuera del alcance no debe ampliarlo.
- **Tomar un 0 como "atasco".** Un 0 solo te atrapa si nada anterior salta por encima de él. En `2, 0, 1` la casilla 0 salta por encima del 0.

## Cuándo usarlo

Úsalo para decidir si una sola pasada de saltos hacia delante puede cruzar un recorrido, como piedras sobre un río o plataformas en un nivel. El problema enlazado, Piedras del río, sigue este patrón.

Una pregunta relacionada pide el **menor número de saltos** para llegar al final. También recorre de izquierda a derecha, pero por capas: todas las casillas alcanzables con un salto, después las alcanzables con dos, y cuenta un salto nuevo cada vez que el recorrido pasa el final de la capa actual.

## Temas relacionados

- [Búsqueda en anchura (BFS)](learn:graphs/bfs): la variante del menor número de saltos es una BFS sobre las casillas, hecha sin estructuras extra.
- [Subir escaleras](learn:dynamic-programming/climbing-stairs): contar las formas de subir en vez de preguntar si se puede.
- [Subarreglo de suma máxima (Kadane)](learn:dynamic-programming/maximum-subarray): otra pasada única que guarda un mejor valor acumulado.
