## La idea

La **búsqueda en profundidad** (DFS) explora un grafo yendo siempre más hondo: desde el nodo actual, salta al primer vecino que aún no se ha visto, y desde ahí a su primer vecino sin ver, y así sucesivamente. Solo cuando un nodo ya no tiene vecinos sin ver, DFS retrocede un paso y prueba el siguiente vecino del nodo anterior.

El ejemplo lo escribe como una función recursiva `visit(u)`. La **pila de llamadas** es el camino actual: los marcos que esperan en ella son justo los nodos desde el inicio hasta el nodo que se está explorando. Cada nodo tiene dos momentos. **Entra** cuando se llama a `visit` con él (se marca como visto y se añade al orden de entrada) y **termina** cuando `visit` retorna, después de que hayan terminado todos sus descendientes. La salida es el orden de entrada seguido del orden de finalización.

### La entrada por defecto

Ocho nodos, no dirigido: una cadena 0 – 1 – 2 – 3 – 4, una segunda cadena 0 – 5 – 6 – 7 y una arista extra 2 – 6, escrita la última. Desde 0, DFS entra en 1, 2, 3 y 4; la pila guarda ahora el camino 0, 1, 2, 3, 4. El nodo 4 no tiene nada nuevo, así que termina, y luego termina 3. De vuelta en 2, el siguiente vecino es 6, así que DFS entra en 6, después en 5 (el primer vecino de 6) y después en 7. Al nodo 5 se llega a través de 2 y 6, no directamente desde 0: cuando 0 mira a 5, ya está visto.

Orden de entrada: 0, 1, 2, 3, 4, 6, 5, 7. Orden de finalización: 4, 3, 5, 7, 6, 2, 1, 0. El nodo de partida siempre termina el último.

## Pasos

1. Marca todos los nodos como no vistos y llama a `visit(0)`.
2. `visit(u)`: marca `u` como visto y apúntalo en el orden de entrada.
3. Para cada vecino `v` de `u`, en el orden de la lista, si `v` no se ha visto, llama a `visit(v)` y espera a que retorne.
4. Cuando se acaban los vecinos, apunta `u` en el orden de finalización y retorna.
5. Escribe el orden de entrada y luego el de finalización.

## Coste

| Operación | Coste |
|---|---|
| Búsqueda completa | `O(n + m)`: cada nodo entra una vez y cada lista se lee una vez |
| Espacio extra | `O(n)`: las marcas de visto y hasta `n` marcos en la pila |

## Errores comunes

- **La profundidad de la recursión.** Un camino largo supone un marco por nodo: una cadena de 10 000 nodos desborda el límite por defecto de Python, de unos 1 000. Sube el límite o pasa a una pila explícita.
- **Una pila explícita no da automáticamente el mismo orden.** Apilar todos los vecinos de golpe y desapilarlos invierte su orden y pierde los momentos de finalización; para imitar la recursión, guarda también en la pila la posición de cada nodo en su lista.
- **Marcar demasiado tarde.** Marca un nodo como visto al entrar en él, o un ciclo hará que DFS dé vueltas para siempre.

## Cuándo usarlo

Usa DFS cuando importa la forma de los caminos más que su longitud: alcanzabilidad, detección de ciclos, orden topológico a partir de los momentos de finalización, componentes conexas, generación de laberintos y búsquedas con vuelta atrás que prueban una opción hasta el final antes de pasar a la siguiente.

## Temas relacionados

- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): la pila que guarda el camino actual.
- [Preorden, inorden y postorden](learn:trees/tree-traversals): en un árbol, los órdenes de entrada y de finalización son el preorden y el postorden.
- [Detección de ciclos](learn:graphs/cycle-detection): DFS con tres colores.
- [Búsqueda en anchura (BFS)](learn:graphs/bfs): la alternativa anillo a anillo.
