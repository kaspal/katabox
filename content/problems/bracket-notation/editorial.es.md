## Enfoque: preorden con dos reglas

Recorre el árbol en preorden y añade trozos a una lista:

1. el valor del nodo;
2. si tiene hijo izquierdo **o** hijo derecho: `(`, el subárbol izquierdo (quizá vacío), `)`;
3. si tiene hijo derecho: `(`, el subárbol derecho, `)`.

La regla 2 escribe `()` para un hijo izquierdo que falta exactamente cuando le sigue un hijo derecho, lo que mantiene el texto sin ambigüedad; un hijo derecho que falta simplemente se omite.

**Complejidad:** O(n) en tiempo y O(n) para el texto (construido en una lista y unido una vez, no con concatenaciones repetidas, que podrían costar O(n²)).
