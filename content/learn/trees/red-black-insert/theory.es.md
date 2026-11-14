## La idea

Un **árbol rojinegro** es un árbol binario de búsqueda que colorea sus enlaces para mantenerse equilibrado. Este tema muestra la variante **inclinada a la izquierda** (LLRB), que se entiende mejor como un **árbol 2-3** dibujado con nodos binarios. Un nodo de un árbol 2-3 guarda una o dos claves, y todas sus hojas están a la misma profundidad. Un enlace **rojo** pega un nodo a su padre para que la pareja funcione como un solo nodo con dos claves; los enlaces negros son aristas normales. Cada nodo guarda el color del enlace que viene de su padre. Los enlaces rojos se inclinan a la izquierda, ningún nodo toca dos enlaces rojos, y todo camino desde la raíz hacia abajo cruza el mismo número de enlaces negros.

Una clave nueva llega como hoja roja. A la vuelta, se hacen tres comprobaciones en cada nodo, en este orden:

1. **Rotar a la izquierda** si el hijo derecho es rojo y el izquierdo no.
2. **Rotar a la derecha** si el hijo izquierdo y su propio hijo izquierdo son rojos.
3. **Invertir colores** si los dos hijos son rojos: pasan a negros y el nodo a rojo. En términos 2-3, un nodo de tres claves se divide y su clave central sube.

Una rotación da a la nueva cima el color de la antigua y pinta de rojo la antigua. Tras cada inserción la raíz se pinta de negro.

El árbol rojinegro clásico del libro CLRS admite enlaces rojos a cualquier lado, así que su reparación tras insertar distingue más casos (un tío rojo o negro, un nieto interior o exterior, y sus simétricos). Inclinarse a la izquierda los reduce a estas tres comprobaciones, a cambio de unas pocas rotaciones de más.

### La entrada por defecto

10, 20, 30, 40, 50, 25, 5. 20 llega rojo a la derecha de 10, así que 10 rota a la izquierda: 20 es la raíz con 10 rojo a su izquierda. 30 da a 20 dos hijos rojos, y la inversión los pone negros. 40 se inclina a la derecha bajo 30 y se rota a la izquierda. 50 da a 40 dos hijos rojos; la inversión pone 40 en rojo, inclinado a la derecha de la raíz, así que 20 rota a la izquierda y 40 pasa a ser la raíz con 20 rojo a su izquierda. 25 y 5 llegan como hijos izquierdos rojos de 30 y 10 y no necesitan corrección. La salida son los valores en preorden, 40, 20, 10, 5, 30, 25, 50, y luego sus colores, 0, 1, 0, 1, 0, 1, 0.

## Pasos

1. Baja como en un ABB (las claves iguales van a la derecha) y engancha la clave como hoja roja nueva.
2. A la vuelta, en cada nodo: si el hijo derecho es rojo y el izquierdo no, rota a la izquierda.
3. Después, si el hijo izquierdo y su hijo izquierdo son rojos, rota a la derecha.
4. Después, si los dos hijos son rojos, invierte sus colores y el del nodo.
5. Tras cada inserción, pinta la raíz de negro.
6. Escribe los valores en preorden y luego sus colores (1 rojo, 0 negro).

## Coste

| Operación | Coste |
|---|---|
| insert | `O(log n)`: un camino de bajada y de vuelta |
| Rotación o inversión | `O(1)` |
| Altura | como mucho unas `2 log n` |
| Espacio extra | `O(n)`: los nodos, con un bit de color cada uno |

## Errores comunes

- **Hacer las comprobaciones en otro orden.** Una rotación a la derecha crea los dos hijos rojos que luego necesita la inversión.
- **Dejar la raíz roja.** Una inversión en la raíz la pinta de rojo; vuelve a pintarla de negro tras cada inserción.
- **Mezclar las variantes.** Un LLRB nunca tiene un enlace rojo a la derecha; el código de CLRS sí lo permite, así que sus reparaciones no se mezclan.

## Cuándo usarlo

Los árboles rojinegros están detrás de muchos mapas ordenados de las bibliotecas, como `TreeMap` de Java y la mayoría de las implementaciones de `std::map`, porque las actualizaciones necesitan pocas rotaciones. La variante inclinada a la izquierda es la más corta de escribir tú mismo.

## Temas relacionados

- [ABB: buscar, insertar, borrar](learn:trees/bst-operations): el árbol simple que los colores mantienen equilibrado.
- [Árbol B: inserción y división](learn:trees/b-tree-insert): el árbol 2-3 que refleja esta variante.
- [Árbol AVL: rotaciones](learn:trees/avl-rotations): las mismas rotaciones, guiadas por alturas.
- [Preorden, inorden y postorden](learn:trees/tree-traversals): el recorrido en preorden que escribe la salida.
