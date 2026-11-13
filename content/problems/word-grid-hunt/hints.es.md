Buscar en la cuadrícula cada palabra por separado repite el mismo trabajo: muchas palabras comparten su principio.
---
Mete todas las palabras en un trie. Así, una búsqueda en profundidad desde cada casilla puede seguir el trie y la cuadrícula a la vez, comprobando de una vez todas las palabras con ese prefijo.
---
DFS(casilla, nodo del trie): para si la letra de la casilla no tiene hijo en el trie; marca la casilla como usada, anota la palabra del hijo si termina ahí, explora los cuatro vecinos y desmarca. Quitar del trie las palabras encontradas (o marcarlas) evita repetirlas.
