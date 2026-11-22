## El error

El código inicial usa el método clásico de `tails`: `tails[k]` es la menor altura con la que puede terminar una elección ascendente de longitud `k + 1`, y `tails` se mantiene ordenada. Para cada altura busca el primer elemento que `h` debe reemplazar, o la añade al final si no hay ninguno. La búsqueda es la equivocada:

```python
i = bisect_right(tails, h)
```

`bisect_right` devuelve la posición justo después de todos los elementos iguales a `h`. Así, cuando `h` ya está en `tails`, cae una posición más allá y reemplaza un valor mayor o alarga `tails`, como si `h` pudiera ir detrás de una altura igual. Eso calcula la elección **no decreciente** más larga, pero la subida tiene que ser estrictamente ascendente.

Con `heights = [2, 2]`: el primer 2 deja `tails = [2]`; para el segundo, `bisect_right([2], 2)` vale 1, que es igual a la longitud, así que se añade y `tails = [2, 2]`. El código devuelve 2, pero dos alturas iguales no pueden escogerse las dos, así que la respuesta es 1.

## La corrección

Busca el primer elemento mayor **o igual** que `h`, para que una altura igual reemplace a su gemela y nunca alargue:

```python
from bisect import bisect_left

i = bisect_left(tails, h)
```

La versión en TypeScript escribe su propia búsqueda binaria; cambia la comparación `tails[mid] <= h` por `tails[mid] < h` para que se detenga en el primer elemento que no sea menor que `h`.

**Complejidad:** O(n log n) en tiempo y O(n) de espacio extra para `tails`.
