## Enfoque: una sola pasada con un mapa hash

Recorre el arreglo una vez. Para cada `nums[i]`, calcula `need = target - nums[i]`. Si `need` ya está en el mapa, devuelve `[map[need], i]`; si no, guarda `nums[i] -> i`.

Buscar antes de guardar es lo que impide usar el mismo elemento dos veces.

### Paso a paso

`nums = [7, -3, 10, 5]`, `target = 2`:

| i | nums[i] | need | mapa antes | resultado |
| --- | --- | --- | --- | --- |
| 0 | 7 | −5 | {} | guarda 7 → 0 |
| 1 | −3 | 5 | {7: 0} | guarda −3 → 1 |
| 2 | 10 | −8 | {7: 0, −3: 1} | guarda 10 → 2 |
| 3 | 5 | −3 | {7: 0, −3: 1, 10: 2} | el −3 está: devuelve [1, 3] |

Con `nums = [9, 2, 9]` y `target = 18`, la pareja del primer 9 es el propio 9, pero en `i = 0` el mapa todavía está vacío, así que un elemento nunca se empareja consigo mismo; el segundo 9 encuentra al primero.

**Complejidad:** O(n) en tiempo y O(n) de espacio extra. Probar todos los pares cuesta O(n²), demasiado lento para 10⁵ números.
