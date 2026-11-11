## Enfoque: una ventana deslizante

Como los costes no son negativos, alargar un tramo nunca lo abarata. Así que recorre las etapas con un extremo derecho y mantén el extremo izquierdo lo más a la izquierda posible:

1. Suma `costs[right]` al total de la ventana.
2. Mientras el total supere `budget`, quita `costs[left]` y mueve `left` un paso a la derecha.
3. Ahora `[left, right]` es el tramo válido más largo que termina en `right`: actualiza la respuesta con `right - left + 1`.

Si una etapa sola supera el presupuesto, el bucle vacía la ventana (`left = right + 1`) y la longitud es 0.

### Paso a paso

`costs = [4, 2, 1, 6, 1, 1]`, `budget = 8`:

| right | total tras sumar | encoger | ventana | longitud |
| --- | --- | --- | --- | --- |
| 0 | 4 | – | [0, 0] | 1 |
| 1 | 6 | – | [0, 1] | 2 |
| 2 | 7 | – | [0, 2] | 3 |
| 3 | 13 | quitar 4, 2 → 7 | [2, 3] | 2 |
| 4 | 8 | – | [2, 4] | 3 |
| 5 | 9 | quitar 1 → 8 | [3, 5] | 3 |

La mejor longitud es 3.

**Complejidad:** O(n) en tiempo (cada etapa entra y sale de la ventana una vez) y O(1) de espacio extra. Probar cada inicio y alargarlo cuesta O(n²) cuando el presupuesto es grande.
