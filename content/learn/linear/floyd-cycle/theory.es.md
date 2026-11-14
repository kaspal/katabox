## La idea

Una lista enlazada puede dar vueltas: el `next` de algún nodo apunta a un nodo anterior y recorrerla no termina nunca. El método de Floyd lo averigua con dos punteros y sin memoria extra. **slow** (la tortuga) avanza un nodo por paso y **fast** (la liebre) avanza dos.

- Si no hay ciclo, fast llega al final de la lista y has terminado.
- Si hay ciclo, los dos punteros acaban dando vueltas por él. Una vez dentro, fast le gana un nodo a slow en cada paso, así que tiene que caer justo en slow; no puede saltárselo. Se **encuentran**.

Una segunda fase encuentra **dónde empieza**: manda slow de vuelta a la cabeza, deja fast en el punto de encuentro y mueve los dos **un** nodo por paso. El nodo donde vuelven a encontrarse es el comienzo del ciclo.

### Por qué funciona la segunda fase

Supón que el tramo antes del ciclo mide `t` nodos y el ciclo mide `c`. Cuando los punteros se encuentran por primera vez, fast ha recorrido el doble que slow, y la distancia de más es un número entero de vueltas. Así que la distancia de slow desde la cabeza también es un número entero de vueltas.

Ahora avanza `t` pasos más desde el punto de encuentro. Sumado a lo que ya recorrió slow, eso da `t` más vueltas enteras: justo donde caerías dando `t` pasos desde la cabeza, que es el comienzo del ciclo. Y un puntero que sale de la cabeza llega a ese comienzo tras los mismos `t` pasos. Los dos llegan a la vez.

## Pasos

1. Pon `slow = fast = head`.
2. Mientras existan `fast` y `fast.next`: mueve `slow` un nodo y `fast` dos. Si son el mismo nodo, para.
3. Si `fast` llegó al final, no hay ciclo: no devuelvas nada.
4. Pon `slow = head`; deja `fast` donde se encontraron.
5. Mientras `slow` no sea `fast`, mueve cada uno un nodo.
6. Devuelve `slow`: el primer nodo del ciclo.

## Coste

| Medida | Coste |
|---|---|
| Detectar el ciclo | `O(n)`: slow alcanza a fast antes de completar una vuelta |
| Encontrar su comienzo | `O(n)`: `t` pasos más |
| Espacio extra | `O(1)`: dos punteros |

## Errores comunes

- **Comprobar solo `fast`.** fast salta dos nodos, así que comprueba `fast` y `fast.next` antes de moverlo.
- **Comparar valores en lugar de nodos.** Dos nodos distintos pueden tener el mismo valor. Compara los propios nodos (`is` en Python, `===` en JavaScript).
- **Reiniciar el puntero equivocado.** Solo slow vuelve a la cabeza; fast tiene que quedarse en el punto de encuentro para que la segunda fase funcione.

## Cuándo usarlo

Úsalo para comprobar si una lista enlazada tiene un bucle sin memoria extra y, en general, en cualquier secuencia en la que cada valor determina el siguiente: generadores pseudoaleatorios o un arreglo cuyos valores son índices dentro de él mismo, donde encuentra un duplicado con espacio `O(1)`. La misma pareja lenta y rápida también encuentra el centro de una lista en una sola pasada.

## Temas relacionados

- [Lista simplemente enlazada](learn:linear/singly-linked-list): la estructura que puede tener un bucle.
- [Dos punteros](learn:searching/two-pointers): dos índices que se mueven con reglas sencillas.
- Tablas hash (próximamente): detectan un ciclo recordando los nodos visitados, con espacio `O(n)`.
