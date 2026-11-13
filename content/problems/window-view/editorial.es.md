## Enfoque: búsqueda en anchura, el último nodo de cada nivel

Procesa el árbol nivel a nivel con una cola. Al empezar cada ronda, la cola tiene exactamente un nivel; saca sus nodos de izquierda a derecha, mete sus hijos y anota el valor del **último** nodo que sale.

### Paso a paso

`[7, 3, 9, 1, 5, null, null, null, 2]`: el nivel 1 es `7`; el nivel 2 es `3, 9` (se ve el 9); el nivel 3 es `1, 5` (se ve el 5); el nivel 4 es `2`. La respuesta es `[7, 9, 5, 2]`.

**Complejidad:** O(n) en tiempo y O(w) de espacio para el nivel más ancho. Un recorrido en profundidad que visite primero los hijos derechos y anote el primer nodo visto en cada profundidad también vale.
