## Enfoque: menores finales y búsqueda binaria

**La idea O(n²).** Sea `best[i]` la elección creciente más larga que termina en la torre `i`: uno más que el mayor `best[j]` con `j < i` y `heights[j] < heights[i]` (o 1). La respuesta es el mayor `best[i]`. Con 5·10⁴ torres son unos 1,25·10⁹ comparaciones: demasiado lento.

**Más rápido.** Mantén `tails`, donde `tails[k]` es la menor altura que puede cerrar una elección creciente de longitud `k + 1` entre las torres vistas. `tails` es estrictamente creciente: una elección de longitud `k + 2` contiene una de longitud `k + 1` cuya última torre es más baja.

Para cada altura `h`, busca la primera posición `p` con `tails[p] ≥ h` (búsqueda binaria, porque `tails` está ordenada):

- si no hay ninguna, `h` alarga la elección más larga: añádela;
- si no, `h` es un final más bajo para las elecciones de longitud `p + 1`: haz `tails[p] = h`.

Usar "≥ h" (y no "> h") es lo que hace estricta la subida: una altura igual sustituye, nunca alarga. La longitud final de `tails` es la respuesta. (`tails` no es en sí una elección válida; solo importa su longitud.)

### Paso a paso

`heights = [5, 1, 8, 3, 6, 9, 4]`:

| h | tails después |
| --- | --- |
| 5 | [5] |
| 1 | [1] |
| 8 | [1, 8] |
| 3 | [1, 3] |
| 6 | [1, 3, 6] |
| 9 | [1, 3, 6, 9] |
| 4 | [1, 3, 4, 9] |

La longitud es 4.

**Complejidad:** O(n log n) en tiempo y O(n) de espacio extra.
