## Enfoque: postorden, devolviendo lo que queda y su total

La regla juzga una rama solo después de sus subramas, con **lo que queda** debajo. Un recorrido en postorden encaja exactamente: poda los hijos y después decide sobre el nodo.

Sea `poda(nodo)` la función que devuelve el subárbol podado y su vigor total:

1. `poda(null)` es `(null, 0)`.
2. Poda los dos hijos: `(izq, a)` y `(der, b)`.
3. El total es `nodo.val + a + b`. Si es negativo, devuelve `(null, 0)`: la rama se corta, y una rama cortada no suma nada a su padre.
4. Si no, haz `nodo.left = izq`, `nodo.right = der` y devuelve `(nodo, total)`.

La respuesta es la primera parte de `poda(root)`.

### Paso a paso

`root = [3, -5, 2, 1, -4]`: `poda(1)` = (1, 1); `poda(-4)` = cortada; `poda(-5)`: -5 + 1 + 0 = -4, cortada; `poda(2)` = (2, 2); la raíz: 3 + 0 + 2 = 5, se conserva solo con su hijo derecho. La salida es `[3, null, 2]`.

**Complejidad:** O(n) en tiempo y O(h) de espacio extra para la recursión (h ≤ 1000 niveles). Recalcular desde cero el total de cada rama, de arriba abajo, repite trabajo: O(n·h).
