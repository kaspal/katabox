## Enfoque: dividir por la raíz con un mapa de índices

El primer valor de un rango del preorden es la raíz de ese subárbol. Búscalo en el rango del inorden: los `L` valores a su izquierda forman el subárbol izquierdo y el resto el derecho. En el preorden, el subárbol izquierdo son entonces los `L` valores siguientes y el derecho los restantes. Recurre en las dos mitades.

Un diccionario de valor a posición en el inorden, construido una vez, hace cada búsqueda O(1); recorrer el rango del inorden para cada raíz cuesta en cambio O(n·h).

### Paso a paso

`preorder = [5, 2, 4, 8, 6]`, `inorder = [2, 4, 5, 6, 8]`: la raíz 5 está en la posición 2 del inorden, así que el subárbol izquierdo tiene preorden `[2, 4]` e inorden `[2, 4]` (el 2 con hijo derecho 4), y el derecho tiene preorden `[8, 6]` e inorden `[6, 8]` (el 8 con hijo izquierdo 6).

**Complejidad:** O(n) en tiempo, O(n) para el mapa y O(h) para la recursión.
