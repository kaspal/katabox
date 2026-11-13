## Enfoque: un árbol de segmentos con desplazamientos perezosos

Construye un árbol de segmentos sobre las estaciones: el nodo `v` cubre un segmento `[lo, hi]` y guarda `mn[v]`, el pronóstico más bajo de ese segmento.

- **Desplazar [a, b] en delta:** si el segmento del nodo está dentro de `[a, b]`, suma `delta` a `mn[v]` y a `lazy[v]` (un desplazamiento que se debe a los hijos) y para. Si solo se solapa, primero **baja** `lazy[v]` a los dos hijos, recurre en ellos y luego haz `mn[v] = min(mn[izq], mn[der])`.
- **Más frío [a, b]:** si el segmento está dentro, devuelve `mn[v]`; si es disjunto, devuelve +∞; si no, baja el pendiente y devuelve la menor de las respuestas de los dos hijos.

Un rango queda cubierto por O(log n) nodos maximales, así que cada operación visita O(log n) nodos.

### Paso a paso

`temps = [5, −2, 7, 0, 3]`, desplazar [1, 3] en +4: los nodos que cubren exactamente 1..3 reciben +4 en `mn` y en `lazy`; sus antecesores recalculan su mínimo. `coldest [1, 3]` lee entonces esos nodos: min(2, 11, 4) = 2.

**Complejidad:** O((n + q) log n) en tiempo y O(n) de espacio. Aplicar cada desplazamiento estación por estación y recorrer cada rango es O(n·q).
