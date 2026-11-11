## Enfoque: una sola pasada con un mapa hash

Recorre el arreglo una vez. Para cada `nums[i]`, calcula `need = target - nums[i]`. Si `need` ya está en el mapa, devuelve `[map[need], i]`; si no, guarda `nums[i] -> i`.

Buscar antes de guardar es lo que impide usar el mismo elemento dos veces.

### Paso a paso

`nums = [3, 2, 4]`, `target = 6`:

| i | nums[i] | need | mapa antes | resultado |
| --- | --- | --- | --- | --- |
| 0 | 3 | 3 | {} | guarda 3 → 0 |
| 1 | 2 | 4 | {3: 0} | guarda 2 → 1 |
| 2 | 4 | 2 | {3: 0, 2: 1} | el 2 está: devuelve [1, 2] |

En `i = 0` la pareja de 3 es el propio 3, pero el mapa todavía está vacío, así que el elemento no se empareja consigo mismo.

**Complejidad:** O(n) en tiempo y O(n) de espacio extra. Probar todos los pares cuesta O(n²), demasiado lento para 10⁴ números.
