## Enfoque: búsqueda en anchura, un nivel por ronda

Una cola recorre el árbol nivel a nivel. Al empezar cada ronda, la cola tiene exactamente los nodos de un nivel:

1. Empieza con la raíz en la cola (un árbol vacío devuelve 0).
2. Lee la longitud `m` de la cola; saca `m` nodos, suma sus valores y mete sus hijos no nulos.
3. Si el total de este nivel es **mayor** que el mejor hasta ahora, guarda el total y el nivel.
4. Repite hasta que la cola esté vacía.

Comparar con `>` y no con `≥` conserva el primer nivel de un empate, el más cercano a la raíz.

### Paso a paso

`root = [4, 7, 2, null, 1, 8, 3]`: la ronda 1 saca el 4 (mejor 4, nivel 1); la ronda 2 saca el 7 y el 2 (9, nivel 2); la ronda 3 saca el 1, el 8 y el 3 (12, nivel 3). La respuesta es 3.

**Complejidad:** O(n) en tiempo y O(w) de espacio extra para el nivel más ancho. Un recorrido en profundidad que suma cada valor al total de su profundidad también vale, en O(n) de tiempo y O(h) de espacio para la recursión.
