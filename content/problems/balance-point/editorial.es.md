## Enfoque: un total y una suma acumulada

La derecha del índice `i` es todo menos la izquierda y la propia caja:

`derecha = total - izquierda - weights[i]`

Así que calcula `total` en una pasada y recorre la fila otra vez con `left = 0`. En cada índice compara `left` con `total - left - weights[i]`; la primera coincidencia es la respuesta. Después de comparar, suma `weights[i]` a `left`.

### Paso a paso

`weights = [4, -1, 6, 2, 1]`, `total = 12`:

| i | izquierda | derecha = 12 − izq − w | ¿equilibra? |
| --- | --- | --- | --- |
| 0 | 0 | 8 | no |
| 1 | 4 | 9 | no |
| 2 | 3 | 3 | **sí** |

**Complejidad:** O(n) en tiempo y O(1) de espacio extra. Sumar los dos lados desde cero para cada índice cuesta O(n²), demasiado lento para 10⁵ cajas.
