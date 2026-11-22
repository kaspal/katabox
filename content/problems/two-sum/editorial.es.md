## Enfoque: una sola pasada con un mapa hash

Recorre el arreglo una vez. Para cada `nums[i]`, calcula `need = target - nums[i]`. Si `need` ya está en el mapa, devuelve `[map[need], i]`; si no, guarda `nums[i] -> i`.

Buscar antes de guardar es lo que impide usar el mismo elemento dos veces.

### Paso a paso

`nums = [-6, 14, 3, 20, -1]`, `target = 14` (Ejemplo 2):

| i | nums[i] | need | mapa antes | resultado |
| --- | --- | --- | --- | --- |
| 0 | −6 | 20 | {} | guarda −6 → 0 |
| 1 | 14 | 0 | {−6: 0} | guarda 14 → 1 |
| 2 | 3 | 11 | {−6: 0, 14: 1} | guarda 3 → 2 |
| 3 | 20 | −6 | {−6: 0, 14: 1, 3: 2} | el −6 está: devuelve [0, 3] |

La línea de 14 necesita un 0, que nunca aparece; no se empareja consigo misma porque busca su pareja antes de guardarse. Con `nums = [9, 2, 9]` y `target = 18`, el primer 9 encuentra el mapa vacío y el segundo 9 encuentra al primero: `[0, 2]`.

**Complejidad:** O(n) en tiempo y O(n) de espacio extra. Probar todos los pares cuesta O(n²), demasiado lento para 10⁵ números.
