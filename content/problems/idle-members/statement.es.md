Un rocódromo guarda a sus socios en `members` y anota cada visita en `visits`, una fila por socio y entrada.

Antes de renovar las cuotas, la gerente quiere contactar con los **socios inactivos**: todos los que no fueron al rocódromo ni una sola vez en noviembre de 2026. Las visitas de octubre, diciembre o cualquier otro mes no cambian eso, y un socio que no ha ido nunca también está inactivo.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `members(id, name)`: `id` es la clave primaria.
- `visits(id, member_id, day)`: `id` es la clave primaria; `member_id` hace referencia a `members(id)`; `day` es la fecha de la visita, escrita `AAAA-MM-DD`.

## Resultado

Devuelve una fila por cada socio sin ninguna visita entre `2026-11-01` y `2026-11-30` (ambos días incluidos), con dos columnas:

| columna | valor |
| --- | --- |
| `id` | el `id` del socio |
| `name` | el nombre del socio |

Ordena las filas por `name` y luego por `id`.

### Ejemplo 1

`members`:

| id | name |
| --- | --- |
| 1 | ana |
| 2 | bo |
| 3 | cy |

`visits`:

| id | member_id | day |
| --- | --- | --- |
| 1 | 1 | 2026-11-03 |
| 2 | 2 | 2026-10-30 |
| 3 | 1 | 2026-11-20 |

Resultado:

| id | name |
| --- | --- |
| 2 | bo |
| 3 | cy |

Ana fue dos veces en noviembre. La única visita de Bo fue en octubre y Cy no fue nunca, así que los dos están inactivos.

### Ejemplo 2

`members`:

| id | name |
| --- | --- |
| 4 | di |

`visits`:

| id | member_id | day |
| --- | --- | --- |
| 5 | 4 | 2026-11-30 |

Resultado: ninguna fila (solo las columnas `id` y `name`). El 30 de noviembre sigue siendo noviembre.

## Restricciones

- 1 ≤ socios ≤ 300, 0 ≤ visitas ≤ 3000.
- Los ids de socios son distintos entre sí, y también los de visitas, cada uno de 1 a 10⁶.
- Un nombre tiene de 1 a 10 letras minúsculas; dos socios pueden llamarse igual.
- Cada visita es de un socio, y cada `day` es una fecha válida de 2026. Un socio puede tener cualquier número de visitas.
