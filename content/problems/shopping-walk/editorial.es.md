## Enfoque: una ventana deslizante con un contador de "faltan"

Cuenta la lista: `want[c]` es cuántos puestos de la letra `c` necesitas, y `missing = need.length`.

Mueve el extremo derecho `r` por los puestos. Al añadir el puesto `c = stalls[r]`: si `have[c] < want[c]`, cubre un artículo más, así que `missing--`; después `have[c]++`.

Mientras `missing = 0`, la ventana `[l, r]` cubre la lista: anota `r − l + 1` y quita el puesto `l`: `have[c]--`, y si eso dejó `have[c] < want[c]`, `missing++`; avanza `l`.

Toda ventana más corta se encuentra: cuando `r` llega a su extremo derecho, el extremo izquierdo avanza exactamente hasta su extremo izquierdo antes de que la ventana deje de cubrir.

### Paso a paso

`stalls = "bcaacbdab"`, `need = "aab"`: la ventana cubre por primera vez en r = 3 ("bcaa", longitud 4); quitar la `b` la rompe. En r = 5 vuelve a cubrir como "caacb" y se encoge a "aacb" (longitud 4) antes de que quitar una `a` la rompa. En r = 7 cubre como "acbda" (longitud 5). La mejor es 4.

**Complejidad:** O(n + m) en tiempo y O(1) de espacio extra (26 contadores). Hacer crecer una ventana desde cada inicio es O(n²).
