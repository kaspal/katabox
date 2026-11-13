## Enfoque: recorrido en orden, parando en k

El recorrido en orden de un árbol de búsqueda visita los valores en orden creciente, así que la respuesta es el k-ésimo nodo que visita.

Con una pila explícita: mete toda la rama izquierda; saca un nodo (el siguiente menor); cuéntalo, y si la cuenta llega a k devuelve su valor; si no, pasa a su hijo derecho y mete la rama izquierda de ese hijo.

### Paso a paso

`[8, 3, 10, 1, 6, null, 14]`, k = 3: se meten 8, 3, 1; sale el 1 (1.º), sale el 3 (2.º), se mete el 6, sale el 6 (3.º): la respuesta es 6.

**Complejidad:** O(h + k) en tiempo y O(h) de espacio, donde h ≤ 1000 es la altura.
