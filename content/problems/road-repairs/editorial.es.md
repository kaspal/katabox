## Enfoque: un arreglo de diferencias

Anota cada obra en sus dos extremos: `change[a] += crews`, y `change[b + 1] −= crews` (omítelo cuando `b + 1 = length`). Después recorre de izquierda a derecha con un total acumulado; en el tramo `i`, el total acumulado es la suma de los operarios de las obras que han empezado y aún no han terminado, que es justo el recuento de ese tramo.

### Paso a paso

`length = 5`, las obras `[0, 2, 1]`, `[1, 4, 2]` y `[3, 3, 5]` dan `change = [1, 2, 0, 4, −5]` (el −1 en 3 y el +5 en 3 se combinan). Los totales acumulados son `[1, 3, 3, 7, 2]`.

**Complejidad:** O(length + repairs) en tiempo y O(length) de espacio. Sumar cada obra tramo a tramo es O(length·repairs).
