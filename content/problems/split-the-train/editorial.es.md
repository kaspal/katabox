## Enfoque: contar y cortar

1. Cuenta los vagones: n.
2. La parte `i` (desde 0) tiene `n // k` vagones, más uno si `i < n mod k`. Así las longitudes difieren como mucho en uno y las partes más largas van primero.
3. Recorre la lista otra vez. Para cada parte, recuerda su primer nodo, ve a su último nodo, corta ahí el enlace `next` y sigue desde el nodo siguiente. Las partes de longitud 0 están vacías (`null`, escrito `[]`).

### Paso a paso

`train = [1..7]`, `k = 3`: n = 7, 7 // 3 = 2 y 7 mod 3 = 1, así que los tamaños son 3, 2, 2. Se corta tras el 3 y tras el 5.

**Complejidad:** O(n + k) en tiempo y O(k) para el resultado. Los nodos se reutilizan.
