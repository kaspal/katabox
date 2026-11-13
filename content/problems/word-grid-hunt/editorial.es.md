## Enfoque: un trie del diccionario y una búsqueda por casilla

1. **Trie:** inserta cada palabra; el nodo donde termina una palabra la guarda.
2. **Búsqueda:** desde cada casilla, lanza una búsqueda en profundidad que recorre la cuadrícula y el trie a la vez. Solo se puede entrar en una casilla si su letra es hija del nodo actual del trie y la casilla no está ya en el camino. Cuando el nuevo nodo del trie guarda una palabra, anótala.
3. **Vuelta atrás:** desmarca la casilla al volver.
4. Ordena las palabras anotadas.

Todas las palabras con el mismo prefijo se comprueban en el mismo recorrido, así que los principios compartidos se exploran una vez. La poda del trie corta un camino en cuanto ninguna palabra lo continúa.

**Complejidad:** como mucho casillas · 4 · 3^(L−1) pasos de camino para palabras de longitud ≤ L, sin depender del número de palabras; más O(longitud total) para construir el trie. Buscar cada palabra por separado repite ese trabajo para cada palabra: con 3·10⁴ palabras que comparten prefijos largos y nunca aparecen, son unos 10⁹-10¹⁰ pasos.
