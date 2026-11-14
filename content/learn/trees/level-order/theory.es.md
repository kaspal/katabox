## La idea

El **recorrido por niveles** visita un árbol fila a fila: la raíz, luego cada nodo un paso por debajo, luego cada nodo dos pasos por debajo, cada fila de izquierda a derecha. Es la **búsqueda en anchura** (BFS) aplicada a un árbol.

La herramienta es una **cola**: el primero en entrar es el primero en salir. Empieza con la raíz en la cola. Después, una y otra vez, saca el nodo del frente, escribe su valor y pon sus hijos (el izquierdo y luego el derecho) al final. Los hijos siempre entran en la cola detrás de todo lo que ya espera, y eso es justo lo que mantiene las filas en orden: los hijos de un nodo solo se escriben cuando ya ha salido toda la fila de arriba.

### El árbol por defecto

Los valores 50, 30, 70, 20, 40, 60, 80, 35 forman un árbol binario de búsqueda (ABB): los menores van a la izquierda y los iguales o mayores a la derecha. 50 es la raíz; 30 y 70 forman la segunda fila; 20, 40, 60 y 80 la tercera; y 35, hijo izquierdo de 40, queda solo en la cuarta. El recorrido por niveles escribe 50, 30, 70, 20, 40, 60, 80, 35.

Fíjate en la cola mientras avanza: tras 50 tiene 30, 70; tras 30 tiene 70, 20, 40; tras 70 tiene toda la tercera fila, cuatro nodos. La cola nunca guarda mucho más que una fila, así que la memoria que necesita el recorrido depende de la **anchura** del árbol, no de su altura.

## Pasos

1. Construye el ABB con los valores y pon la raíz en una cola vacía.
2. Mientras la cola no esté vacía, saca el nodo del frente y escribe su valor.
3. Si el nodo tiene hijo izquierdo, añádelo al final de la cola.
4. Si el nodo tiene hijo derecho, añádelo al final de la cola.
5. Cuando la cola se vacía, todos los nodos se han escrito, fila a fila.

## Coste

| Operación | Coste |
|---|---|
| Recorrido completo | `O(n)`: cada nodo entra y sale de la cola una vez |
| Espacio extra | `O(w)` para el nivel más ancho, hasta unos `n/2` en un árbol completo |

## Errores comunes

- **Usar el principio de una lista como cola.** En Python, `list.pop(0)` desplaza todos los valores y cuesta `O(n)`; usa `collections.deque`. En JavaScript, `shift()` tiene el mismo problema con arreglos grandes, así que mejor guarda un índice de cabeza.
- **Perder dónde acaba cada fila.** Si necesitas cada fila por separado (por ejemplo, su suma), apunta la longitud de la cola al empezar una fila y saca exactamente esos nodos.

## Cuándo usarlo

Usa el recorrido por niveles cuando la respuesta dependa de la profundidad: imprimir un árbol fila a fila, el nodo más a la izquierda o más a la derecha de cada fila, la fila más ancha o el nodo más cercano que cumpla algo (BFS llega antes a los más cercanos).

## Temas relacionados

- [Cola y búfer circular](learn:linear/queue-ring-buffer): la cola que mueve el recorrido.
- [Preorden, inorden y postorden](learn:trees/tree-traversals): los recorridos en profundidad del mismo árbol.
- [Diámetro de un árbol](learn:trees/tree-diameter): BFS en un árbol para encontrar su camino más largo.
- Búsqueda en anchura en grafos (próximamente): la misma idea cuando a un nodo se puede llegar por más de un camino.
