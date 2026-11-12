## Enfoque: un trie con contadores de paso

Un **trie** guarda cadenas letra a letra: la raíz es el prefijo vacío, y el hijo de un nodo por la letra `c` es ese prefijo seguido de `c`. Los títulos con un comienzo común comparten los nodos de ese comienzo.

Dale a cada nodo un contador `pasos`. Insertar un título recorre desde la raíz sus letras, crea los hijos que falten y suma 1 a `pasos` en cada nodo al que llega. El `pasos` de un nodo es entonces el número de títulos cuyo camino pasa por él, que es el número de títulos que **empiezan por** su prefijo.

```
insertar(título):
    nodo = raíz
    para c en título:
        nodo = hijo(nodo, c)   # se crea si falta
        nodo.pasos += 1
contar(texto):
    nodo = raíz
    para c en texto:
        si no hay hijo(nodo, c): devolver 0
        nodo = hijo(nodo, c)
    devolver nodo.pasos
```

Los títulos repetidos vuelven a pasar por los mismos nodos, así que cada copia cuenta.

### Paso a paso

Con "moon", "moonlight", "mars", "moon", "map", por el nodo de "m" se pasa 5 veces, por "mo" 3 (los dos "moon" y "moonlight") y por "moonl" una. La búsqueda "x" se detiene en la raíz: no hay hijo "x", así que 0.

**Complejidad:** O(total de letras de los títulos) para construirlo y O(longitud) por búsqueda, en total O(T + Q), con O(T) nodos. Comparar cada título con cada texto escrito cuesta O(N · Q · L).
