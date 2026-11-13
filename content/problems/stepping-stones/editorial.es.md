## Enfoque: seguir la piedra alcanzable más lejana

Las piedras alcanzables siempre forman un prefijo `0..reach`: si la piedra `i` es alcanzable, también lo es cada piedra anterior del camino (un salto desde `j` puede caer en cualquier piedra hasta `j + jumps[j]`).

Recorre de izquierda a derecha con `reach = 0`. En cada piedra `i`, si `i > reach`, el prefijo terminó antes de `i` y la última piedra no se puede alcanzar. Si no, extiende `reach = max(reach, i + jumps[i])`. Si el recorrido pasa la última piedra, la respuesta es true.

### Paso a paso

Ejemplo 1, `[2, 1, 0, 3, 1]`: la piedra 0 da alcance 2, la 1 da alcance 2, la 2 no añade nada y la 3 queda más allá del alcance 2: false. Ejemplo 2, `[2, 0, 3, 0, 0, 1]`: alcance 2, 2 y luego 5 en la piedra 2, que es la última.

**Complejidad:** O(n) en tiempo y O(1) de espacio. Una búsqueda que prueba cada longitud de salto desde cada piedra alcanzable es O(n · salto máximo), O(n²) con saltos largos.
