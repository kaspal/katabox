## Enfoque: bajar un intervalo abierto por el árbol

La definición habla de subárboles enteros, así que cada nodo hereda límites de todos sus antecesores. Recorre el árbol con un intervalo `(bajo, alto)`, empezando en `(−∞, +∞)`:

- el valor del nodo debe cumplir `bajo < val < alto`;
- su hijo izquierdo recibe `(bajo, val)` y el derecho `(val, alto)`.

Si todos los nodos cumplen, el árbol es de búsqueda.

**Comprobación equivalente:** un recorrido en orden (izquierda, nodo, derecha) de un árbol de búsqueda da los valores en orden estrictamente creciente; compara cada valor visitado con el anterior.

### Paso a paso

`[8, 3, 10, 1, 9]`: el nodo 9 es hijo derecho del 3, que es hijo izquierdo del 8, así que su intervalo es `(3, 8)`. 9 no es menor que 8: no es válido.

**Complejidad:** O(n) en tiempo y O(h) para la recursión (h ≤ 1000).
