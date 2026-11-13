## Enfoque: tres punteros, una pasada

Mantén la cinta en cuatro zonas:

| `0 .. low−1` | `low .. mid−1` | `mid .. high` | `high+1 .. n−1` |
| --- | --- | --- | --- |
| vidrio | papel | sin mirar | plástico |

Empieza con `low = mid = 0` y `high = n − 1`, y repite mientras `mid ≤ high`:

- `items[mid] = 0`: intercámbialo con `items[low]` (que es papel, o el mismo objeto) y después `low++`, `mid++`;
- `items[mid] = 1`: `mid++`;
- `items[mid] = 2`: intercámbialo con `items[high]` y después `high--`. `mid` no se mueve, porque el objeto que llegó desde `high` es nuevo.

Cada paso reduce la zona sin mirar en uno, así que el bucle termina tras n pasos como mucho con la cinta ordenada.

### Paso a paso

`[2, 0, 1, 2, 0]`: el plástico de la posición 0 se intercambia con el último objeto → `[0, 0, 1, 2, 2]`, high = 3; vidrio, vidrio y papel avanzan `low` y `mid`; en mid = 3 hay plástico: se intercambia consigo mismo, high = 2, y el bucle termina.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra.
