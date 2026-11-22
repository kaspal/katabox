## Idea

En el problema base el trie se construía una vez; aquí cambia, y un contador por nodo lo hace fácil. Cada nodo guarda `count`, el número de títulos del catálogo cuyo camino pasa por él, que es justo el número de títulos que empiezan por el prefijo que forma ese nodo. `"add"` recorre las letras del título, creando nodos cuando hace falta, y suma 1 a cada nodo del camino; `"remove"` recorre el mismo camino y resta 1 (el título está seguro, así que ningún contador baja de 0). `"count"` sigue las letras escritas y devuelve el contador del último nodo, o 0 si alguna letra no tiene hijo. Los nodos cuyo contador llega a 0 pueden quedarse: simplemente responden 0.

## Complejidad

O(L) por operación, donde L ≤ 20 es la longitud de la palabra: O(q · L) en tiempo en total, y como mucho O(q · L) nodos en el trie.
