## La idea

Una **cola de prioridad** entrega los valores de menor a mayor, lleguen como lleguen. Un **montículo binario** lo hace de forma barata. Es un árbol binario guardado en un simple arreglo: la raíz está en la posición 0 y los hijos de la posición `i` están en `2i + 1` y `2i + 2` (así que su padre es `(i − 1) // 2`). El árbol se llena fila a fila, así que no se guardan enlaces.

Este es un **montículo de mínimos**: cada padre es como mucho igual que sus hijos, así que el menor valor siempre está en la raíz.

- **push**: pon el valor al final del arreglo y luego **súbelo**: mientras sea menor que su padre, intercámbialos.
- **pop**: toma la raíz. Lleva el último valor a la raíz y luego **húndelo**: mientras sea mayor que su hijo menor, intercámbialo con ese hijo. Con el montículo vacío, pop devuelve `−1`.

### La entrada por defecto

push 42, 17, 8, 23, 4 deja el arreglo como 4, 8, 17, 42, 23. El último push muestra la subida: 4 empieza en la posición 4, gana a su padre 23, pasa a la posición 1, gana a 8 y se convierte en la raíz.

El primer pop toma 4 y lleva 23 a la raíz. Sus hijos son 8 y 17; el menor es 8, así que 23 se intercambia con 8 y se para encima de 42. Cinco pop devuelven 4, 8, 17, 23, 42: los valores en orden.

## Pasos

1. push `x`: añade `x` al final; sea `i` su posición.
2. Mientras `i > 0` y `x` sea menor que su padre, intercámbialos y lleva `i` al padre.
3. pop: si el montículo está vacío, devuelve `−1`. Si no, devuelve la raíz y quita el último valor.
4. Si quedan valores, pon ese último valor en la raíz.
5. Mientras tenga algún hijo y sea mayor que su hijo menor, intercámbialo con ese hijo.

## Coste

| Operación | Coste |
|---|---|
| push | `O(log n)`: un camino hacia arriba |
| pop | `O(log n)`: un camino hacia abajo |
| Consultar el menor | `O(1)`: está en la posición 0 |
| Espacio extra | `O(n)`, el propio arreglo, sin enlaces |

## Errores comunes

- **Comparar con el hijo equivocado.** Al hundir hay que intercambiar con el hijo *menor*; con el otro quedaría un valor mayor encima de uno menor.
- **Hijos más allá del final.** Un nodo puede tener dos hijos, uno o ninguno. Comprueba `2i + 1 < size` antes de leer un hijo.
- **Esperar un arreglo ordenado.** Solo la raíz está garantizada; el arreglo 4, 8, 17, 42, 23 es un montículo válido pero no está ordenado.

## Cuándo usarlo

Usa un montículo siempre que necesites una y otra vez el menor (o el mayor) de un conjunto que cambia: planificar tareas por prioridad, mezclar listas ordenadas, quedarte con las k mejores puntuaciones o los caminos más cortos de Dijkstra. Python tiene `heapq` (de mínimos); en JavaScript escribes uno como el del ejemplo.

## Temas relacionados

- [Ordenación por montículo (Heap Sort)](learn:sorting/heap-sort): un montículo de máximos construido en el sitio y vaciado en un arreglo ordenado.
- [Cola y búfer circular](learn:linear/queue-ring-buffer): la cola normal, primero en entrar, primero en salir.
- [ABB: buscar, insertar, borrar](learn:trees/bst-operations): un árbol totalmente ordenado, a un coste mayor.
- Caminos más cortos de Dijkstra (próximamente): una cola de prioridad en acción.
