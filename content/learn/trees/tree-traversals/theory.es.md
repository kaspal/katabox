## La idea

Un **árbol binario** está hecho de **nodos**; cada nodo guarda un valor y enlaza con dos hijos como mucho, uno izquierdo y otro derecho. El nodo de arriba es la **raíz**. Aquí los valores forman un **árbol binario de búsqueda (ABB)**: cada valor nuevo empieza en la raíz y va a la izquierda si es menor que el nodo, a la derecha si es igual o mayor, hasta encontrar un sitio libre. Los valores por defecto 50, 30, 70, 20, 40, 60, 80 forman un árbol completo de tres niveles, con 50 en la raíz.

**Recorrer** un árbol es visitar cada nodo una vez. Lo natural es hacerlo de forma recursiva: visitar un nodo, su subárbol izquierdo y su subárbol derecho, en algún orden. Lo único que cambia es *cuándo* escribes el valor del propio nodo:

- **Preorden**: primero el nodo, luego la izquierda, luego la derecha. 50, 30, 20, 40, 70, 60, 80.
- **Inorden**: la izquierda, luego el nodo, luego la derecha. 20, 30, 40, 50, 60, 70, 80.
- **Postorden**: la izquierda, luego la derecha, luego el nodo. 20, 40, 30, 60, 80, 70, 50.

### Por qué el inorden sale ordenado

En un ABB, todo lo que hay en el subárbol izquierdo de un nodo es menor que él y nada de su subárbol derecho lo es. El inorden escribe el subárbol izquierdo entero, luego el nodo y luego el subárbol derecho entero, así que los valores salen en orden ascendente.

El preorden escribe un padre antes que sus hijos, que es el orden que necesitas para copiar un árbol o imprimirlo como un esquema. El postorden escribe primero los hijos, justo lo que necesitas cuando la respuesta de un nodo depende de sus hijos: tamaños, alturas, o borrar un árbol desde las hojas hacia arriba.

## Pasos

1. Inserta los valores uno a uno en un ABB, empezando por un árbol vacío.
2. Preorden(nodo): si el nodo existe, escribe su valor, luego haz el preorden del hijo izquierdo y después el del derecho.
3. Inorden(nodo): inorden del hijo izquierdo, escribe el valor, inorden del hijo derecho.
4. Postorden(nodo): postorden del hijo izquierdo, luego del derecho, y después escribe el valor.
5. La salida tiene los tres órdenes uno detrás de otro.

## Coste

| Operación | Coste |
|---|---|
| Cada recorrido | `O(n)`: cada nodo se escribe una vez |
| Espacio extra | `O(h)` de pila de llamadas para un árbol de altura `h` |

## Errores comunes

- **Olvidar el caso vacío.** Todo recorrido recursivo tiene que parar en un hijo que no existe, o leerá un valor de la nada.
- **Recursión profunda.** Un árbol construido con valores ordenados es un único camino largo, así que la recursión llega a `n` llamadas y puede desbordar la pila. Usa una pila explícita para árboles muy profundos.

## Cuándo usarlo

Usa el preorden para copiar o serializar un árbol, el inorden para leer un ABB en orden y el postorden siempre que un nodo necesite antes los resultados de sus hijos, como al calcular tamaños de subárboles o liberar memoria.

## Temas relacionados

- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): la pila que usa todo recorrido.
- [Recorrido por niveles](learn:trees/level-order): visitar el mismo árbol fila a fila con una cola.
- [ABB: buscar, insertar, borrar](learn:trees/bst-operations): cómo se construye y se modifica el árbol.
- [Pila](learn:linear/stack): cómo recorrer sin recursión.
