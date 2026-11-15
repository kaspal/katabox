Un food truck anota cada pedido que vende en `orders`: cuándo se hizo, al minuto, y su total en céntimos.

Al terminar la temporada, la dueña quiere un **resumen diario**: para cada día en que el camión recibió pedidos, cuántos pedidos hubo y cuánto dinero sumaron. Los días sin ningún pedido no forman parte del resumen.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `orders(id, placed, total_cents)`: `id` es la clave primaria; `placed` es el momento en que se hizo el pedido, escrito `AAAA-MM-DD HH:MM`; `total_cents` es el total del pedido en céntimos.

El día de un pedido son los 10 primeros caracteres de `placed` (`AAAA-MM-DD`).

## Resultado

Devuelve una fila por cada día con al menos un pedido, con tres columnas:

| columna | valor |
| --- | --- |
| `day` | el día, escrito `AAAA-MM-DD` |
| `orders` | el número de pedidos hechos ese día |
| `revenue_cents` | la suma de `total_cents` de esos pedidos |

Ordena las filas por `day`, del más antiguo al más reciente.

### Ejemplo 1

`orders`:

| id | placed | total_cents |
| --- | --- | --- |
| 1 | 2026-05-01 12:05 | 850 |
| 2 | 2026-05-01 12:05 | 1200 |
| 3 | 2026-05-01 19:40 | 640 |
| 4 | 2026-05-03 13:10 | 990 |

Resultado:

| day | orders | revenue_cents |
| --- | --- | --- |
| 2026-05-01 | 3 | 2690 |
| 2026-05-03 | 1 | 990 |

Los tres pedidos del 1 de mayo caen el mismo día aunque se hicieron a dos horas distintas, así que forman una sola fila: 850 + 1200 + 640 = 2690. El 2 de mayo no hubo pedidos y no aparece.

### Ejemplo 2

`orders`: vacía (ninguna fila).

Resultado: vacío (ninguna fila).

## Restricciones

- 0 ≤ pedidos ≤ 3000.
- Los ids son distintos, de 1 a 10⁶.
- `placed` es un momento válido de 2026, escrito `AAAA-MM-DD HH:MM`; varios pedidos pueden compartir el mismo minuto.
- 1 ≤ `total_cents` ≤ 10⁶.
