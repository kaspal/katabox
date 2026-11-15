Una pequeña biblioteca de préstamo guarda a sus socios en `members` y cada préstamo en `loans`. Cada préstamo tiene una fecha de devolución prevista y la fecha en que el libro volvió, o `NULL` mientras sigue prestado.

La bibliotecaria quiere la lista de **devoluciones tardías**: cada préstamo devuelto *después* de su fecha prevista. Un libro devuelto el mismo día previsto llega a tiempo, y un libro que aún no ha vuelto no aparece en la lista.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `members(id, name)`: `id` es la clave primaria.
- `loans(id, member_id, title, due, returned)`: `member_id` hace referencia a `members(id)`; `due` (fecha prevista) y `returned` (fecha de devolución) se escriben `AAAA-MM-DD`; `returned` es `NULL` mientras el libro está prestado.

## Resultado

Devuelve una fila por préstamo tardío, con tres columnas:

| columna | valor |
| --- | --- |
| `name` | el nombre del socio |
| `title` | el título del libro |
| `days_late` | los días completos entre `due` y `returned` |

Ordena las filas por `days_late`, de mayor a menor, luego por `title` y luego por el `id` del préstamo.

### Ejemplo 1

`members`:

| id | name |
| --- | --- |
| 1 | ana |
| 2 | bo |
| 3 | cy |

`loans`:

| id | member_id | title | due | returned |
| --- | --- | --- | --- | --- |
| 10 | 1 | Dune | 2026-03-01 | 2026-03-04 |
| 11 | 2 | Emma | 2026-03-02 | 2026-03-02 |
| 12 | 3 | Ulysses | 2026-03-03 | NULL |
| 13 | 2 | Beloved | 2026-03-01 | 2026-03-04 |
| 14 | 1 | Middlemarch | 2026-03-05 | 2026-03-01 |

Resultado:

| name | title | days_late |
| --- | --- | --- |
| bo | Beloved | 3 |
| ana | Dune | 3 |

Emma volvió el día previsto, Ulysses sigue prestado y Middlemarch volvió antes. Beloved y Dune llevan 3 días de retraso, así que se ordenan por título.

### Ejemplo 2

`members`: `(1, zed)`, `(2, amy)`. `loans`: `(1, 1, Odyssey, 2026-01-01, 2026-01-11)` y `(2, 2, Iliad, 2026-01-01, 2026-01-02)`.

Resultado: `(zed, Odyssey, 10)` y luego `(amy, Iliad, 1)`.

## Restricciones

- 1 ≤ socios ≤ 200, 0 ≤ préstamos ≤ 2000.
- Cada fecha es un día válido de 2026.
- Un socio puede tomar prestado el mismo título más de una vez.
