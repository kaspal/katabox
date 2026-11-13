## Enfoque: dividir por la raíz con un mapa de índices

El primer valor de un rango del preorden es la raíz de ese subárbol. Búscalo en el rango del inorden: los `L` valores a su izquierda forman el subárbol izquierdo y el resto el derecho. En el preorden, el subárbol izquierdo son entonces los `L` valores siguientes y el derecho los restantes. Recurre en las dos mitades.

Un diccionario de valor a posición en el inorden, construido una vez, hace cada búsqueda O(1); recorrer el rango del inorden para cada raíz cuesta en cambio O(n·h).

### Paso a paso

`preorder = [3, 9, 20, 15, 7]`, `inorder = [9, 3, 15, 20, 7]`: la raíz 3 está en la posición 1 del inorden, así que el subárbol izquierdo es `[9]` y el derecho tiene preorden `[20, 15, 7]` e inorden `[15, 20, 7]`, cuya raíz es el 20 con los hijos 15 y 7.

**Complejidad:** O(n) en tiempo, O(n) para el mapa y O(h) para la recursión.
