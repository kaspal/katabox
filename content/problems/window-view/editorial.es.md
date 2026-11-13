## Enfoque: búsqueda en anchura, el último nodo de cada nivel

Procesa el árbol nivel a nivel con una cola. Al empezar cada ronda, la cola tiene exactamente un nivel; saca sus nodos de izquierda a derecha, mete sus hijos y anota el valor del **último** nodo que sale.

### Paso a paso

`[1, 2, 3, null, 5, null, 4]`: el nivel 1 es `1`; el nivel 2 es `2, 3` (se ve el 3); el nivel 3 es `5, 4` (se ve el 4). La respuesta es `[1, 3, 4]`.

**Complejidad:** O(n) en tiempo y O(w) de espacio para el nivel más ancho. Un recorrido en profundidad que visite primero los hijos derechos y anote el primer nodo visto en cada profundidad también vale.
